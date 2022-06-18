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

    pub fn apply_range(self, left: ValueOrRange, right: ValueOrRange)-> ValueOrRange{
        match left {
            ValueOrRange::Value(l) => match right{
                ValueOrRange::Value(r) => ValueOrRange::Value(self.apply(l, r)),
                ValueOrRange::Range { start, end } => 
                ValueOrRange::Range { start: self.apply(l, start), end: self.apply(l, end) },
            },
            ValueOrRange::Range { start, end } =>
            match right {
                ValueOrRange::Value(r) => 
                ValueOrRange::Range { start: self.apply( start, r), end: self.apply(end, r) },
                
                //TODO this should really create a compound range
                ValueOrRange::Range { start:start2, end:end2 } => 
                ValueOrRange::Range { start: self.apply( start, start2), end: self.apply(end,end2) },
            },
        }
    }

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