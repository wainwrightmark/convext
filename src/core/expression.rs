use std::fmt::Binary;
use std::{collections::BTreeMap, default, str::FromStr};

use crate::core::prelude::*;
use itertools::Itertools;
use num::traits::ops::inv;
use pest::iterators::{Pairs, Pair};
use pest::Parser;
use pest_derive::Parser;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Ord, PartialOrd, Clone, Copy, Serialize, Deserialize)]
pub enum BinaryOperator{
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
    GEq
}

impl BinaryOperator{
    pub fn apply(self, left: f32, right: f32)-> f32{
        match self {
            BinaryOperator::Add => left + right,
            BinaryOperator::Sub => left - right,
            BinaryOperator::Mul => left * right,
            BinaryOperator::Div => left / right,
            BinaryOperator::And => if left != 0.0 && right != 0.0 {1.0} else{0.0},
            BinaryOperator::Or => if left != 0.0 || right != 0.0 {1.0} else{0.0},
            BinaryOperator::Eq => if left == right {1.0} else{0.0},
            BinaryOperator::Neq => if left != right {1.0} else{0.0},
            BinaryOperator::Lt => if left < right {1.0} else{0.0},
            BinaryOperator::Gt => if left > right {1.0} else{0.0},
            BinaryOperator::LEq => if left <= right {1.0} else{0.0},
            BinaryOperator::GEq => if left >= right {1.0} else{0.0},
        }
    }
}

impl FromStr for BinaryOperator{
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

            _=>  Err(format!("Could not parse {} as binary operator", s))
        }
    }
}

#[derive(PartialEq, Eq, Ord, PartialOrd, Clone, Copy, Serialize, Deserialize)]
pub enum UnaryOperator{
    Sub,
    Abs,
    Sig
}

impl FromStr for UnaryOperator{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "sub" => Ok(UnaryOperator::Sub),
            "abs" => Ok(UnaryOperator::Abs),
            "sig" => Ok(UnaryOperator::Sig),
            _=>  Err(format!("Could not parse {} as unary operator", s))
        }
    }
}

impl UnaryOperator{
    pub fn apply(self, value: f32)-> f32{
        match self {
            UnaryOperator::Sub => -value ,
            UnaryOperator::Abs => value.abs(),
            UnaryOperator::Sig => value.signum(),
        }
    }
}

#[derive(PartialEq, PartialOrd, Clone, Serialize, Deserialize)]
pub enum Expression {
    Number { val: f32 },
    Variable { name: String },
    Unary {operator: UnaryOperator, operand: Box<Expression>},
    Binary { left: Box<Expression>,  operator: BinaryOperator, right: Box<Expression>},
}

impl Expression {

    pub fn fold(self)-> Self{
        match self {
            Expression::Number { val } => self,
            Expression::Variable { name } => Self::Variable { name },
            Expression::Unary { operator, operand } => {
                let o = operand.fold();
                if let Expression::Number { val: v } = o{
                    return Expression::Number { val: operator.apply(v) };
                }
                Self::Unary { operator, operand: o.into() }
            },
            Expression::Binary { left, operator, right } => {
                let l = left.fold();                
                let r = right.fold();
                if let Expression::Number { val: vl } = l{
                    
                    if let Expression::Number { val: vr } = r{
                        return Expression::Number { val: operator.apply(vl, vr) };
                    }                    
                }
                Expression::Binary { left: l.into(), operator, right:r.into() }
            },
        }
    }

    pub fn try_get_value(&self, grammar: &Grammar) -> Result<f32, String> {
        match self {
            Expression::Number { val } => Ok(*val),
            Expression::Variable { name } => grammar.defs
                .get(&name.to_ascii_lowercase())
                .ok_or(format!("Varaible '{}' not defined", name))
                .map(|&x| x),
            Expression::Unary { operator, operand } =>{
                let v = operand.try_get_value(grammar)?;
                Ok(operator.apply(v))
            } ,
            Expression::Binary { left, operator, right } => todo!(),
        }
    }

    pub fn parse(next: Pair<Rule>)->Self{
        let rule = next.as_rule();

        match rule {

            Rule::expression =>{
                Self::parse(next.into_inner().next().unwrap()) //We need to go deeper
            }
            Rule::simple_expression =>{
                Self::parse(next.into_inner().next().unwrap())
            }

            Rule::number => {
                let val = next.as_str().parse::<f32>().unwrap();
                Expression::Number { val }
            }
            Rule::variable => {
                let name = next.as_str().replacen('?', "", 1);
                Expression::Variable { name }
            }
            Rule::unary =>{
                let mut inner = next.into_inner();
                let operator = inner.next().unwrap().as_str().parse::<UnaryOperator>().unwrap();
                let operand = Self:: parse(inner.next().unwrap()).into();

                Expression::Unary { operator, operand }.fold()

            }
            
            Rule::binary =>{
                let mut inner = next.into_inner();
                let left = Self:: parse(inner.next().unwrap()).into();
                let operator = inner.next().unwrap().as_str().parse::<BinaryOperator>().unwrap();
                let right = Self:: parse(inner.next().unwrap()).into();

                Expression::Binary { left, operator, right }.fold()
            }

            _ => {
                unreachable!("unexpected rule {:?}", rule)
            },
        }
    }
}