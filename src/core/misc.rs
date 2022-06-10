use std::{collections::BTreeMap, default, str::FromStr};

use crate::core::prelude::*;
use itertools::Itertools;
use num::traits::ops::inv;
use pest::iterators::Pairs;
use pest::Parser;
use pest_derive::Parser;
use serde::{Deserialize, Serialize};
use yew::Properties;

#[derive(Parser)]
#[grammar = "core/convext.pest"]
pub struct ConvextParser;

pub fn parse(input: &str) -> Result<Grammar, String> {
    let mut file_pairs = ConvextParser::parse(Rule::file, input).map_err(|e| e.to_string())?;
    let file = file_pairs.next().unwrap();

    let mut defs = BTreeMap::<String, f32>::default();
    let mut temp_rules = BTreeMap::<String, UserRule>::default();

    let mut temp_top_level = Vec::<Invocation>::default();

    for pair in file.into_inner() {
        match pair.as_rule() {
            Rule::statement => {
                let statement = pair.into_inner().next().unwrap();

                match statement.as_rule() {
                    Rule::invocation => {
                        let ii = Invocation::try_parse(&mut statement.into_inner())?;
                        temp_top_level.push(ii);
                    }
                    Rule::rule => {
                        let mut inner = statement.into_inner();
                        let rule_keyword = inner.next();
                        let name = inner.next().unwrap().as_str().to_string();

                        let mut invocations = Vec::<Invocation>::new();

                        for p in inner {
                            match p.as_rule() {
                                Rule::EOI => (),
                                Rule::keyword_end => (),
                                Rule::invocation => {
                                    let ti = Invocation::try_parse(&mut p.into_inner())?;
                                    invocations.push(ti);
                                }
                                _ => unreachable!(),
                            }
                        }

                        let inserted = temp_rules
                            .insert(
                                name.to_ascii_lowercase(),
                                UserRule {
                                    name: name.clone(),
                                    children: invocations,
                                },
                            )
                            .is_none();
                        if !inserted {
                            return Err(format!("Variable '{}' defined more than once", name));
                        }
                    }
                    Rule::assignment => {
                        let mut inner = statement.into_inner();
                        let let_keyword = inner.next();
                        let name = inner.next().unwrap().as_str().to_string();
                        let val_string: String = inner
                            .next()
                            .unwrap()
                            .as_str()
                            .chars()
                            .map(|c| match c {
                                'm' => '-',
                                'M' => '_',
                                _ => c,
                            })
                            .collect();
                        let val = val_string.parse::<f32>().unwrap();

                        let inserted = defs.insert(name.to_ascii_lowercase(), val).is_none();
                        if !inserted {
                            return Err(format!("Variable '{}' defined more than once", name));
                        }
                    }

                    _ => unreachable!(),
                }
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    for rule_name in temp_top_level.iter().filter_map(|i| {
        if let Method::Rule(r) = i.method.clone() {
            Some(r)
        } else {
            None
        }
    }) {
        if !temp_rules.contains_key(&rule_name) {
            return Err(format!("Rule '{}' does not exist", rule_name));
        }
    }

    for (_, rule) in temp_rules.iter() {
        for rule_name in rule.children.iter().filter_map(|i| {
            if let Method::Rule(r) = i.method.clone() {
                Some(r)
            } else {
                None
            }
        }) {
            if !temp_rules.contains_key(&rule_name) {
                return Err(format!("Rule '{}' does not exist", rule_name));
            }
        }
    }

    Ok(Grammar {
        defs,
        rules: temp_rules,
        top_level: temp_top_level,
    })
}

#[derive(PartialEq, PartialOrd, Clone, Serialize, Deserialize)]
pub enum Method {
    Root,
    Primitive(Primitive),
    Rule(String),
}

#[derive(PartialEq, PartialOrd, Clone, Serialize, Deserialize)]
pub struct Invocation {
    pub method: Method,
    pub properties: Vec<TempProperty>,
}

impl Invocation {
    pub fn get_children(
        &self,
        absolute_properties: &NodeProperties,
        grammar: &Grammar,
    ) -> Vec<Node> {
        match self.method.clone() {
            Method::Primitive(_) => Default::default(),
            Method::Root => unreachable!(),
            Method::Rule(r) => grammar
                .rules
                .get(&r)
                .unwrap()
                .children
                .iter()
                .map(|c| c.to_node(absolute_properties, grammar))
                .collect_vec(),
        }
    }

    pub fn to_node(&self, parent_properties: &NodeProperties, grammar: &Grammar) -> Node {
        Node {
            invocation: self.clone(),
            absolute_properties: parent_properties
                .make_absolute(&NodeProperties::from_temp(&self.properties, &grammar.defs)),
            children: None,
        }
    }

    pub fn try_parse(invocation: &mut Pairs<Rule>) -> Result<Self, String> {
        let method_name = invocation.next().unwrap().as_str().to_ascii_lowercase();

        let method = Primitive::from_str(&method_name)
            .ok()
            .map(Method::Primitive)
            .unwrap_or(Method::Rule(method_name));

        let mut properties = Vec::<TempProperty>::new();

        for pair in invocation {
            let mut inner = pair.into_inner();
            let prop = TempProperty::try_parse(&mut inner)?;
            properties.push(prop);
        }

        Ok(Self { method, properties })
    }
}

#[derive(PartialEq, PartialOrd, Clone, Serialize, Deserialize)]
pub struct UserRule {
    pub name: String,
    pub children: Vec<Invocation>,
}

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
    ) -> ExpandStatistics {
        let mut stats = ExpandStatistics::default();

        if self.children.is_some() {
            for child in self.children.as_mut().unwrap().iter_mut() {
                let child_stats = child.expand_once(settings, grammar);
                stats = stats + &child_stats;
            }
        } else {
            let new_children = self
                .invocation
                .get_children(&self.absolute_properties, grammar)
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
