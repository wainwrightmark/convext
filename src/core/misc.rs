use std::{collections::BTreeMap, default, str::FromStr};

use crate::core::prelude::*;
use itertools::Itertools;
use num::traits::ops::inv;
use pest::iterators::Pairs;
use pest::Parser;
use pest_derive::Parser;
use serde::{Deserialize, Serialize};

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

    for rule_name in temp_top_level.iter().filter_map(|i|if let Method::Rule(r) = i.method.clone() {Some(r)} else{None} ) {        if !temp_rules.contains_key(&rule_name)            
        {
            return Err(format!("Rule '{}' does not exist", rule_name));
        }
    }

    for (_, rule) in temp_rules.iter() {
        for rule_name in rule.children.iter().filter_map(|i|if let Method::Rule(r) = i.method.clone() {Some(r)} else{None} ) {
            if !temp_rules.contains_key(&rule_name)            
        {
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

//pub const PRIMITIVES: [&str; 2] = ["square", "circle"];

#[derive(PartialEq, PartialOrd, Clone, Serialize, Deserialize)]
pub enum TempValue {
    Number { val: f32 },
    Variable { name: String },
}

impl TempValue {
    pub fn try_get_value(&self, defs: &BTreeMap<String, f32>) -> Result<f32, String> {
        match self {
            TempValue::Number { val } => Ok(*val),
            TempValue::Variable { name } => defs
                .get(&name.to_ascii_lowercase())
                .ok_or(format!("Varaible '{}' not defined", name))
                .map(|&x| x),
        }
    }
}


#[derive(PartialEq, PartialOrd, Clone, Serialize, Deserialize)]
pub enum Method{
    Root,
    Primitive(Primitive),
    Rule(String)

}


#[derive(PartialEq, PartialOrd, Clone, Serialize, Deserialize)]
pub struct Invocation {
    pub method: Method,
    pub properties: TempProperties,
}

impl Invocation {
    pub fn get_children(&self, absolute_properties: &Properties, grammar: &Grammar) -> Vec<Node> {

        match self.method.clone() {
            Method::Primitive(_) => Default::default(),
            Method::Root => unreachable!(), 
            Method::Rule(r) => grammar.rules
            .get(&r)
            .unwrap()
            .children
            .iter()
            .map(|c| c.to_node(absolute_properties, grammar))
            .collect_vec(),
        
        }
    }

    pub fn to_node(&self, parent_properties: &Properties, grammar: &Grammar) -> Node {
        Node {
            invocation: self.clone(),
            absolute_properties: parent_properties
                .make_absolute(&self.properties.try_convert(&grammar.defs).unwrap()),
            children: None,
        }
    }

    pub fn try_parse(invocation: &mut Pairs<Rule>) -> Result<Self, String> {
        let method_name = invocation.next().unwrap().as_str().to_ascii_lowercase();

        let method = Primitive::from_str(&method_name).ok().map(|p|Method::Primitive(p)) .unwrap_or(Method::Rule(method_name));

        let properties_vec = invocation
            .map(|p| TempProperty::parse(&mut p.into_inner()))
            .collect_vec();

        let properties = properties_vec.try_into()?;

        Ok(Self { method, properties })
    }
}

#[derive(PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct Grammar {
    pub top_level: Vec<Invocation>,
    pub defs: BTreeMap<String, f32>,
    pub rules: BTreeMap<String, UserRule>,
}

impl Grammar {
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


#[derive(PartialEq, PartialOrd, Clone, Serialize, Deserialize)]
pub struct UserRule {
    pub name: String,
    pub children: Vec<Invocation>,
}


#[derive(PartialEq, PartialOrd, Clone)]
pub struct Node {
    pub invocation: Invocation,
    pub absolute_properties: Properties,
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

    pub fn to_svg_element(&self, grammar: &Grammar) -> String {
        let relative_properties = self
            .invocation
            .properties
            .try_convert(&grammar.defs)
            .unwrap();

        if self.children.is_some() && !self.children.as_ref().unwrap().is_empty() {
            let child_text = self
                .children
                .as_ref()
                .unwrap()
                .iter()
                .map(|c| c.to_svg_element(grammar))
                .join("\r\n");

            format!("<g style=\"transform:  translate({x}px, {y}px) scale({p}%) rotate({r}deg);\">\r\n {child_text}\r\n </g>",

            x= relative_properties.x,
            y =   relative_properties.y,
            r = relative_properties.r,
            p =  relative_properties.p * 100.0,
                    //no color
            
            child_text = child_text)
        } else {
            

            match self.invocation.method{
                Method::Root => "".to_string(),
                Method::Primitive(p) => p.to_svg(&relative_properties, &self.absolute_properties),
                Method::Rule(_) => "".to_string(),
            }
        }
    }

    pub fn expand_once(&mut self, settings: ExpandSettings, grammar: &Grammar) -> ExpandStatistics {
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
                    //let node = c.invocation.to_node(&self.absolute_properties, grammar);
                    if settings.should_cull(&node) {
                        stats.nodes_culled += 1;
                        None
                    } else {
                        stats.new_nodes += 1;
                        Some(node)
                    }
                })
                .collect_vec();

            // .rule {
            //     Command::User { rule } => rule
            //         .children
            //         .iter()
            //         .filter_map(|c| {
            //             let node = c.to_node(&self.absolute_properties);
            //             if settings.should_cull(&node) {
            //                 stats.nodes_culled += 1;
            //                 None
            //             } else {
            //                 stats.new_nodes += 1;
            //                 Some(node)
            //             }
            //         })
            //         .collect_vec(),
            //     _ => Default::default(),
            // };

            self.children = Some(new_children);
        };

        stats
    }
}

#[derive(Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExpandStatistics {
    new_nodes: usize,
    nodes_culled: usize,
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
    max_nodes: usize,
    max_depth: usize,
    min_a: f32,
    min_p: f32,
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


