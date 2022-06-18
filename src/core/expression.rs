use std::fmt::Binary;
use std::{collections::BTreeMap, default, str::FromStr};

use crate::core::prelude::*;
use itertools::Itertools;
use num::traits::ops::inv;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;
use rand::Rng;
use rand::prelude::StdRng;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, PartialOrd, Clone, Serialize, Deserialize)]
pub enum ExpressionOrRange {
    Range {
        is_random: bool,
        first: Expression,
        second: Expression,
    },
    Exp(Expression),
}

impl ExpressionOrRange {
    pub fn get_variables(&self) -> Box<dyn Iterator<Item = String>> {
        match self {
            ExpressionOrRange::Range {
                is_random,
                first,
                second,
            } => Box::new(first.get_variables().chain(second.get_variables())),
            ExpressionOrRange::Exp(e) => e.get_variables(),
        }
    }

    pub fn try_get_value(
        &self,
        grammar: &Grammar,
        context: &NodeProperties,
        rng: &mut StdRng,
    ) -> Result<ValueOrRange, String> {

        match self {
            ExpressionOrRange::Range { is_random, first, second } =>{

                let start = first.try_get_value(grammar, context, rng)?.min_value();
                let end = second.try_get_value(grammar, context, rng)?.max_value();

                if *is_random{
                    let v =if start <= end{
                        rng.gen_range(start..=end)
                        
                    }else{
                        rng.gen_range(end..=start)
                    };
                    Ok(ValueOrRange::Value(v))
                }else{
                    Ok(ValueOrRange::Range { start, end }) 
                }
            },
            ExpressionOrRange::Exp(e) => e.try_get_value(grammar, context, rng),
        }
    }

    pub fn parse(next: Pair<Rule>) -> Result<Self, String> {
        let rule = next.as_rule();

        match rule {
            Rule::expression =>{
                let exp = Expression::parse(next)?;
                    let r = ExpressionOrRange::Exp(exp);
                    Ok(r)
            },
            Rule::range => {
                let mut inner2 = next.into_inner();
                let first = Expression::parse(inner2.next().unwrap())?;
                //let dots = inner2.next();
                let second = Expression::parse(inner2.next().unwrap())?;
                let r = ExpressionOrRange::Range {
                    is_random: false,
                    first,
                    second,
                };
                Ok(r)
            }
            Rule::range_random => {
                let mut inner2 = next.into_inner();
                let first = Expression::parse(inner2.next().unwrap())?;
                //let dots = inner2.next();
                let second = Expression::parse(inner2.next().unwrap())?;
                let r = ExpressionOrRange::Range {
                    is_random: true,
                    first,
                    second,
                };
                Ok(r)
            }

            Rule::expression_or_range => {
                let mut inner = next.into_inner();
                let next = inner.next().unwrap();

                Self::parse(next)
            }
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq, PartialOrd, Clone, Serialize, Deserialize)]
pub enum Expression {
    Number {
        val: f32,
    },
    Variable {
        name: String,
    },
    PropertyAccess {
        property: PropertyKey,
    },
    Unary {
        operator: UnaryOperator,
        operand: Box<Expression>,
    },
    Binary {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
}

impl Expression {
    pub fn get_variables(&self) -> Box<dyn Iterator<Item = String>> {
        match self {
            Expression::Unary { operator, operand } => return operand.get_variables(),
            Expression::Binary {
                left,
                operator,
                right,
            } => return Box::new(left.get_variables().chain(right.get_variables())),
            Expression::Variable { name } => {
                return Box::new(std::iter::once::<String>(name.to_string()))
            }
            _ => return Box::new(std::iter::empty::<String>()),
        };
    }

    pub fn fold(self) -> Self {
        match self {
            Expression::Number { val } => self,
            Expression::Variable { name } => Self::Variable { name },
            Expression::PropertyAccess { property } => Self::PropertyAccess { property },
            Expression::Unary { operator, operand } => {
                let o = operand.fold();
                if let Expression::Number { val: v } = o {
                    return Expression::Number {
                        val: operator.apply(v),
                    };
                }
                Self::Unary {
                    operator,
                    operand: o.into(),
                }
            }

            Expression::Binary {
                left,
                operator,
                right,
            } => {
                let l = left.fold();
                let r = right.fold();
                if let Expression::Number { val: vl } = l {
                    if let Expression::Number { val: vr } = r {
                        return Expression::Number {
                            val: operator.apply(vl, vr),
                        };
                    }
                }
                Expression::Binary {
                    left: l.into(),
                    operator,
                    right: r.into(),
                }
            }
        }
    }

    pub fn try_get_value(
        &self,
        grammar: &Grammar,
        context: &NodeProperties,
        rng: &mut StdRng,
    ) -> Result<ValueOrRange, String> {
        match self {
            Expression::Number { val } => Ok(ValueOrRange::Value(*val)),
            Expression::Variable { name } => grammar
                .defs
                .get(&name.to_ascii_lowercase())
                .ok_or(format!("Varaible '{}' not defined", name))
                .map(|&x| ValueOrRange::Value(x)),
            Expression::Unary { operator, operand } => {
                let val_or_range = operand.try_get_value(grammar, context, rng)?;

                match val_or_range {
                    ValueOrRange::Value(v) => Ok(ValueOrRange::Value(operator.apply(v))),
                    ValueOrRange::Range { start, end } => Ok(ValueOrRange::Range {
                        start: operator.apply(start),
                        end: operator.apply(end),
                    }),
                }
            }

            Expression::PropertyAccess { property } => Ok(property.get(context)),
            Expression::Binary {
                left,
                operator,
                right,
            } => {
                let l = left.try_get_value(grammar, context, rng)?;
                let r = right.try_get_value(grammar, context, rng)?;

                Ok(operator.apply_range(l, r))
            }
        }
    }

    pub fn parse(next: Pair<Rule>) -> Result<Self, String> {
        let rule = next.as_rule();

        match rule {
            Rule::expression => {
                Self::parse(next.into_inner().next().unwrap()) //We need to go deeper
            }
            Rule::simple_expression => Self::parse(next.into_inner().next().unwrap()),

            Rule::number => {
                let val = next.as_str().parse::<f32>().unwrap();
                Ok(Expression::Number { val })
            }
            Rule::variable => {
                let name = next.as_str().replacen('?', "", 1);
                Ok(Expression::Variable { name })
            }

            Rule::property_access => {
                let name = next.as_str().replacen('?', "", 1);
                let property = PropertyKey::from_str(name.as_str())?;
                Ok(Expression::PropertyAccess { property })
            }
            Rule::unary => {
                let mut inner = next.into_inner();
                let operator = inner
                    .next()
                    .unwrap()
                    .as_str()
                    .parse::<UnaryOperator>()
                    .unwrap();
                let operand = Self::parse(inner.next().unwrap())?.into();

                Ok(Expression::Unary { operator, operand }.fold())
            }

            Rule::binary => {
                let mut inner = next.into_inner();
                let left = Self::parse(inner.next().unwrap())?.into();
                let operator = inner
                    .next()
                    .unwrap()
                    .as_str()
                    .parse::<BinaryOperator>()
                    .unwrap();
                let right = Self::parse(inner.next().unwrap())?.into();

                Ok(Expression::Binary {
                    left,
                    operator,
                    right,
                }
                .fold())
            }

            _ => {
                unreachable!("unexpected rule {:?}", rule)
            }
        }
    }
}
