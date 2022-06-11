use std::{collections::BTreeMap, default, str::FromStr};

use crate::core::prelude::*;
use itertools::Itertools;
use num::traits::ops::inv;
use pest::iterators::Pairs;
use pest::Parser;
use pest_derive::Parser;
use rand::{prelude::StdRng, Rng};
use serde::{Deserialize, Serialize};
use yew::Properties;

#[derive(PartialEq, PartialOrd, Clone, Serialize, Deserialize)]
pub struct UserRule {
    pub name: String,
    pub cases: Vec<RuleCase>
}

#[derive(PartialEq, PartialOrd, Clone, Serialize, Deserialize)]
pub struct RuleCase{
    pub probability: Value,
    pub invocations: Vec<Invocation>, 
}

impl RuleCase{
    pub fn should_enter(&self, grammar:&Grammar, rng:&mut StdRng)->bool{
        let prob = self.probability.try_get_value(&grammar.defs).unwrap();

        if prob >= 1.0{
            return true;
        }
        else if prob <= 0.0{
            return false;
        }
        else {
            let enter= rng.gen_bool(prob.into());
            return enter;
        }


    }
}