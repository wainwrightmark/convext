use std::{collections::BTreeMap, default, str::FromStr};

use crate::core::prelude::*;
use itertools::Itertools;
use num::traits::ops::inv;
use pest::iterators::Pairs;
use pest::Parser;
use pest_derive::Parser;
use rand::prelude::StdRng;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct Grammar {
    pub top_level: Vec<Invocation>,
    pub defs: BTreeMap<String, f32>,
    pub rules: BTreeMap<String, UserRule>,
}

impl Grammar {
    pub fn get_variables(&self) -> Vec<(String, Option<PropertyType>)> {
        let rule_invocations = self.rules.values().flat_map(|z| z.cases.iter().flat_map(|c|c.invocations.iter()));                
        let all_invocations = self.top_level.iter().chain (rule_invocations);

        let all_properties = all_invocations.flat_map(|i| i.properties.iter());

        let prob_properties = self.rules.values().flat_map(|r|r.cases
            .iter()
        .map(|c|c.probability.clone())
        .flat_map(|p| {
            if let Some(Expression::Variable { name } )= p {
                Some((name, PropertyType::UnitInterval))
            } else {
                None
            }
        })
    );

        all_properties
            .flat_map(|p| {
                if let Expression::Variable { name } = p.value.clone() {
                    Some((name, p.key.get_type()))
                } else {
                    None
                }
            })
            .chain(prob_properties)
            .sorted_by_key(|p| p.0.clone())
            .group_by(|p| p.0.clone())
            .into_iter()
            .map(|x| {
                let n = x.1.map(|p| p.1).sorted().dedup().take(2).collect_vec();
                if n.len() == 1 {
                    (x.0, Some(n[0]))
                } else {
                    (x.0, None)
                }
            })
            .collect_vec()
    }

    pub fn override_defs(&mut self, new_defs: &BTreeMap<String, f32>) {
        for (key, val) in new_defs {
            self.defs.insert(key.to_ascii_lowercase(), *val);
        }
    }

    pub fn expand(&self, settings: &ExpandSettings, rng:&mut StdRng,) -> Node {
        let mut current = ExpandStatistics::default();
        let nodes = self
            .top_level
            .iter()
            .map(|i| i.to_node(&NodeProperties::default_initial(), self))
            .collect_vec();

        let mut root = Node {
            invocation: Invocation {
                method: Method::Root,
                properties: Default::default(),
            },
            absolute_properties: NodeProperties::default_initial(),
            children: Some(nodes),
        };
        loop {
            let changes = root.expand_once(settings, self, rng);

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
        } else if node.absolute_properties.p * node.absolute_properties.w < self.min_p {
            true
        } else if node.absolute_properties.p * node.absolute_properties.l < self.min_p {
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
            max_depth: 20,
            min_a: 0.001,
            min_p: 0.001,
        }
    }
}
