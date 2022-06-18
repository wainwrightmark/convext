use std::{collections::BTreeMap, default, str::FromStr};

use crate::core::prelude::*;
use itertools::Itertools;
use num::traits::ops::inv;
use pest::iterators::Pairs;
use pest::Parser;
use pest_derive::Parser;
use rand::prelude::StdRng;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ExpandSettings {
    pub max_nodes: usize,
    pub max_depth: usize,
    pub min_a: f32,
    pub min_p: f32,
}

impl ExpandSettings {
    ///Should this node be culled, according to the settings
    pub fn should_cull(&self, node: &Node) -> bool {
        if node.absolute_properties.a.max_value() < self.min_a
            || node.absolute_properties.d > self.max_depth
            || node.absolute_properties.p.max_value() * node.absolute_properties.w.max_value() < self.min_p
            || node.absolute_properties.p.max_value() * node.absolute_properties.l.max_value() < self.min_p
            || node.absolute_properties.x.max_abs() - node.absolute_properties.p.min_value() > 1.5
        {
            true
        } else {
            node.absolute_properties.y.max_abs() - node.absolute_properties.p.min_value() > 1.5
        }
    }
}