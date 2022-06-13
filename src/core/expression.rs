use std::fmt::Binary;
use std::{collections::BTreeMap, default, str::FromStr};

use crate::core::prelude::*;
use itertools::Itertools;
use num::traits::ops::inv;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Ord, PartialOrd, Clone, Copy, Serialize, Deserialize)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,

    And,
    Or,

    Eq,
    Neq,
    Lt,
    Gt,
    LEq,
    GEq,
}

impl BinaryOperator {
    pub fn apply(self, left: f32, right: f32) -> f32 {
        match self {
            BinaryOperator::Add => left + right,
            BinaryOperator::Sub => left - right,
            BinaryOperator::Mul => left * right,
            BinaryOperator::Div => left / right,
            BinaryOperator::And => {
                if left != 0.0 && right != 0.0 {
                    1.0
                } else {
                    0.0
                }
            }
            BinaryOperator::Or => {
                if left != 0.0 || right != 0.0 {
                    1.0
                } else {
                    0.0
                }
            }
            BinaryOperator::Eq => {
                if left == right {
                    1.0
                } else {
                    0.0
                }
            }
            BinaryOperator::Neq => {
                if left != right {
                    1.0
                } else {
                    0.0
                }
            }
            BinaryOperator::Lt => {
                if left < right {
                    1.0
                } else {
                    0.0
                }
            }
            BinaryOperator::Gt => {
                if left > right {
                    1.0
                } else {
                    0.0
                }
            }
            BinaryOperator::LEq => {
                if left <= right {
                    1.0
                } else {
                    0.0
                }
            }
            BinaryOperator::GEq => {
                if left >= right {
                    1.0
                } else {
                    0.0
                }
            }
        }
    }
}

impl FromStr for BinaryOperator {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "add" => Ok(BinaryOperator::Add),
            "sub" => Ok(BinaryOperator::Sub),
            "mul" => Ok(BinaryOperator::Mul),
            "div" => Ok(BinaryOperator::Div),

            "and" => Ok(BinaryOperator::Add),
            "or" => Ok(BinaryOperator::Or),

            "eq" => Ok(BinaryOperator::Eq),
            "neq" => Ok(BinaryOperator::Neq),
            "lt" => Ok(BinaryOperator::Lt),
            "gt" => Ok(BinaryOperator::Gt),
            "leq" => Ok(BinaryOperator::LEq),
            "geq" => Ok(BinaryOperator::GEq),

            "+" => Ok(BinaryOperator::Add),
            "-" => Ok(BinaryOperator::Sub),
            "*" => Ok(BinaryOperator::Mul),
            "/" => Ok(BinaryOperator::Div),

            "&&" => Ok(BinaryOperator::Add),
            "||" => Ok(BinaryOperator::Or),

            "==" => Ok(BinaryOperator::Eq),
            "!=" => Ok(BinaryOperator::Neq),
            "<" => Ok(BinaryOperator::Lt),
            ">" => Ok(BinaryOperator::Gt),
            "<=" => Ok(BinaryOperator::LEq),
            ">=" => Ok(BinaryOperator::GEq),

            _ => Err(format!("Could not parse {} as binary operator", s)),
        }
    }
}

#[derive(PartialEq, Eq, Ord, PartialOrd, Clone, Copy, Serialize, Deserialize)]
pub enum UnaryOperator {
    Sub,
    Abs,
    Sig,
}

impl FromStr for UnaryOperator {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "sub" => Ok(UnaryOperator::Sub),
            "-" => Ok(UnaryOperator::Sub),
            "abs" => Ok(UnaryOperator::Abs),
            "sig" => Ok(UnaryOperator::Sig),
            _ => Err(format!("Could not parse {} as unary operator", s)),
        }
    }
}

impl UnaryOperator {
    pub fn apply(self, value: f32) -> f32 {
        match self {
            UnaryOperator::Sub => -value,
            UnaryOperator::Abs => value.abs(),
            UnaryOperator::Sig => value.signum(),
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
    ) -> Result<f32, String> {
        match self {
            Expression::Number { val } => Ok(*val),
            Expression::Variable { name } => grammar
                .defs
                .get(&name.to_ascii_lowercase())
                .ok_or(format!("Varaible '{}' not defined", name))
                .map(|&x| x),
            Expression::Unary { operator, operand } => {
                let v = operand.try_get_value(grammar, context)?;
                Ok(operator.apply(v))
            }

            Expression::PropertyAccess { property } => Ok(property.get(context)),
            Expression::Binary {
                left,
                operator,
                right,
            } => {
                let l = left.try_get_value(grammar, context)?;
                let r = right.try_get_value(grammar, context)?;
                Ok(operator.apply(l, r))
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
