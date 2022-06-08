use std::{collections::BTreeMap, default, str::FromStr};

use crate::core::prelude::*;
use itertools::Itertools;
use num::traits::ops::inv;
use pest::iterators::Pairs;
use pest::Parser;
use pest_derive::Parser;
use serde::{Deserialize, Serialize};


#[derive(PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct Grammar {
    pub top_level: Vec<Invocation>,
    pub defs: BTreeMap<String, f32>,
    pub rules: BTreeMap<String, UserRule>,
}

impl Grammar {

    pub fn override_defs(mut self, new_defs: BTreeMap<String, f32>){

        for (key, val) in new_defs{
            self.defs.insert(key, val);
        }
    }

    pub fn expand(&self, settings: ExpandSettings) -> Node {
        let mut current = ExpandStatistics::default();
        let nodes = self
            .top_level
            .iter()
            .map(|i| i.to_node(&Properties::default_initial(), self))
            .collect_vec();

        let mut root = Node {
            invocation: Invocation {
                method: Method::Root,
                properties: Default::default(),
            },
            absolute_properties: Properties::default_initial(),
            children: Some(nodes),
        };
        loop {
            let changes = root.expand_once(settings, self);

            current = current + &changes;
            if changes.new_nodes == 0 {
                break;
            }
            if current.new_nodes >= settings.max_nodes {
                break;
            }
        }

        root
    }
}

#[derive(Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExpandStatistics {
    pub new_nodes: usize,
    pub nodes_culled: usize,
}

impl std::ops::Add<&ExpandStatistics> for ExpandStatistics {
    type Output = ExpandStatistics;

    fn add(self, rhs: &ExpandStatistics) -> Self::Output {
        Self {
            new_nodes: self.new_nodes + rhs.new_nodes,
            nodes_culled: self.nodes_culled + rhs.nodes_culled,
        }
    }
}

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
        if node.absolute_properties.a < self.min_a {
            true
        } else if node.absolute_properties.d > self.max_depth {
            true
        } else if node.absolute_properties.p < self.min_p {
            true
        } else if node.absolute_properties.x.abs() - node.absolute_properties.p > 1.5 {
            true
        } else {
            node.absolute_properties.y.abs() - node.absolute_properties.p > 1.5
        }
    }
}

impl Default for ExpandSettings {
    fn default() -> Self {
        Self {
            max_nodes: 1000,
            max_depth: 10,
            min_a: 0.01,
            min_p: 0.001,
        }
    }
}