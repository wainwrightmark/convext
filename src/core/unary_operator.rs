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