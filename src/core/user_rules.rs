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
    pub cases: Vec<RuleCase>,
}

#[derive(PartialEq, PartialOrd, Clone, Serialize, Deserialize)]
pub struct RuleCase {
    pub probability: Option<Expression>,
    pub invocations: Vec<Invocation>,
}

impl RuleCase {
    pub fn should_enter(
        &self,
        grammar: &Grammar,
        context: &NodeProperties,
        rng: &mut StdRng,
    ) -> bool {
        if let Some(value) = &self.probability {
            let prob = value.try_get_value(grammar, context, rng).unwrap();
            if prob.min_value() >= 1.0 {
                true
            } else if prob.max_value() <= 0.0 {
                false
            } else {
                rng.gen_bool(prob.max_value().into())//todo look at this
            }
        } else {
            true
        }
    }
}
