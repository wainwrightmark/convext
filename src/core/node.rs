
use std::{collections::BTreeMap, default, str::FromStr};

use crate::core::prelude::*;
use itertools::Itertools;
use num::traits::ops::inv;
use pest::iterators::Pairs;
use pest::Parser;
use pest_derive::Parser;
use rand::prelude::StdRng;
use serde::{Deserialize, Serialize};
use yew::Properties;

#[derive(PartialEq, PartialOrd, Clone)]
pub struct Node {
    pub invocation: Invocation,
    pub absolute_properties: NodeProperties,
    pub children: Option<Vec<Node>>,
}

impl Node {
    pub fn to_svg(&self, grammar: &Grammar) -> String {
        let elements = self.to_svg_element(grammar);

        format!(
            "<svg viewbox=\"-1 -1 2 2\" width=\"100%\" height=\"100%\" > {} </svg>",
            elements
        )
    }

    fn get_style(rp: &NodeProperties) -> String {
        let mut transform = "".to_string();
        if rp.x != 0.0 || rp.y != 0.0 {
            transform = format!("{} translate({x}px, {y}px) ", transform, x = rp.x, y = rp.y);
        }
        if rp.p != 1.0 {
            transform = format!("{} scale({p}%) ", transform, p = rp.p * 100.0);
        }
        if (rp.r != 0.0) {
            transform = format!("{} rotate({r}deg)", transform, r = rp.r);
        }
        if !transform.is_empty() {
            format!("style=\"transform: {};\"", transform)
        } else {
            "".to_string()
        }
    }

    pub fn to_svg_element(&self, grammar: &Grammar) -> String {
        let relative_properties =
            NodeProperties::from_temp(&self.invocation.properties, &grammar.defs);

        if self.children.is_some() && !self.children.as_ref().unwrap().is_empty() {
            let child_text = self
                .children
                .as_ref()
                .unwrap()
                .iter()
                .map(|c| c.to_svg_element(grammar))
                .join("\r\n");

            let style = Self::get_style(&relative_properties);

            format!(
                "<g {style}>\r\n {child_text}\r\n </g>",
                //no color
                style = style,
                child_text = child_text
            )
        } else {
            match self.invocation.method {
                Method::Root => "".to_string(),
                Method::Primitive(p) => p.to_svg(&relative_properties, &self.absolute_properties),
                Method::Rule(_) => "".to_string(),
            }
        }
    }

    pub fn expand_once(
        &mut self,
        settings: &ExpandSettings,
        grammar: &Grammar,
        rng:&mut StdRng,
    ) -> ExpandStatistics {
        let mut stats = ExpandStatistics::default();

        if self.children.is_some() {
            for child in self.children.as_mut().unwrap().iter_mut() {
                let child_stats = child.expand_once(settings, grammar, rng);
                stats = stats + &child_stats;
            }
        } else {
            let new_children = self
                .invocation
                .get_children(&self.absolute_properties, grammar, rng)
                .into_iter()
                .filter_map(|node| {
                    if settings.should_cull(&node) {
                        stats.nodes_culled += 1;
                        None
                    } else {
                        stats.new_nodes += 1;
                        Some(node)
                    }
                })
                .collect_vec();

            self.children = Some(new_children);
        };

        stats
    }
}