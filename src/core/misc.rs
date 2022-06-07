use std::{
    collections::BTreeMap,
    default,
};

use crate::core::prelude::*;
use itertools::Itertools;
use num::traits::ops::inv;
use pest::iterators::Pairs;
use pest::Parser;
use pest_derive::Parser;
use serde::{Serialize, Deserialize};

#[derive(Parser)]
#[grammar = "core/convext.pest"]
pub struct ConvextParser;

pub fn parse(input: &str) -> Result<Grammar, String> {
    let mut file_pairs = ConvextParser::parse(Rule::file, input).map_err(|e| e.to_string())?;
    let file = file_pairs.next().unwrap();

    let mut defs = BTreeMap::<String, f32>::default();
    let mut temp_rules = BTreeMap::<String, UserRule>::default();

    let mut temp_top_level = Vec::<TempInvocation>::default();

    for pair in file.into_inner() {
        match pair.as_rule() {
            Rule::statement => {
                let statement = pair.into_inner().next().unwrap();

                match statement.as_rule() {
                    Rule::invocation => {
                        let ii = TempInvocation::parse(&mut statement.into_inner());
                        temp_top_level.push(ii);
                    }
                    Rule::rule => {
                        let mut inner = statement.into_inner();
                        let rule_keyword = inner.next();
                        let name = inner.next().unwrap().as_str().to_string();

                        let invocations = inner
                            .filter_map(|p| match p.as_rule() {
                                Rule::EOI => None,
                                Rule::keyword_end => None,
                                Rule::invocation => {
                                    Some(TempInvocation::parse(&mut p.into_inner()))
                                }
                                _ => unreachable!(),
                            })
                            .collect_vec();

                        let inserted = 
                        temp_rules
                        .insert(name.to_ascii_lowercase(), UserRule { name: name.clone(), children: invocations}).is_none();
                    if !inserted{
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

                        let inserted = defs
                            .insert(name.to_ascii_lowercase(), val).is_none();
                        if !inserted{
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

    for invocation in temp_top_level.iter() {
        if !temp_rules.contains_key(&invocation.rule.to_ascii_lowercase())&&!PRIMITIVES.contains(&invocation.rule.to_ascii_lowercase().as_str())  {
            return Err(format!("Rule '{}' does not exist", invocation.rule));
        }
    }
    
    for (_, rule) in temp_rules.iter() {

        for invocation in rule.children.iter(){
            if !temp_rules.contains_key(&invocation.rule.to_ascii_lowercase()) &&!PRIMITIVES.contains(&invocation.rule.to_ascii_lowercase().as_str()) {
                return Err(format!("Rule '{}' does not exist", invocation.rule));
            }
        }
        
    }

    // let mut temp_rules2: BTreeMap<String, (UserRule, Rc<UserRule>)> = Default::default();

    // for key in temp_rules.keys() {
    //     let rule = UserRule {
    //         name: key.clone(),
    //         children: Default::default(),
    //     };
    //     temp_rules2.insert(key.to_ascii_lowercase(), (rule, Rc::new(rule)));
    // }

    // let mut rules: BTreeMap<String, Rc<UserRule>> = Default::default();

    // for (name, invocations) in temp_rules{
    //     let rule = temp_rules2.get(&name.to_ascii_lowercase()).unwrap();
    // }

    // let top_level_result : Result<Vec<Invocation>, String> = temp_top_level.into_iter().map(|i| i.try_into(&rules, &defs)).collect();
    // let top_level = top_level_result?;

    Ok(Grammar {
        defs,
        rules : temp_rules,
        top_level : temp_top_level,
    })
}

pub const PRIMITIVES: [&str; 2] = ["square", "circle"];

#[derive(PartialEq, PartialOrd, Clone, Serialize, Deserialize)]
pub enum TempValue {
    Number { val: f32 },
    Variable { name: String },
}

impl TempValue{
    pub fn try_get_value(&self, defs: &BTreeMap<String, f32>) -> Result<f32, String>{

match self {
    TempValue::Number { val } => Ok(*val),
    TempValue::Variable { name } => {
        defs.get(&name.to_ascii_lowercase()).ok_or(format!("Varaible '{}' not defined", name)).map(|&x|x)
    },
}

    }   
}

#[derive(PartialEq, PartialOrd, Clone, Serialize, Deserialize)]
pub struct TempProperty {
    pub name: String,
    pub val: TempValue,
}

impl TempProperty {    
    pub fn parse(property: &mut Pairs<Rule>) -> Self {
        let name = property.next().unwrap().as_str().to_string();

        let next = property.next().unwrap().into_inner().next().unwrap();

        let rule = next.as_rule();

        let val = match rule {

            Rule::number => {
                let val_string: String = next
                    .as_str()
                    .chars()
                    .map(|c| match c {
                        'm' => '-',
                        'M' => '_',
                        _ => c,
                    })
                    .collect();
                let val = val_string.parse::<f32>().unwrap();
                TempValue::Number { val }
            }
            Rule::variable => {
                
                let name = next.as_str().replacen('?', "", 1);
                TempValue::Variable { name }
            }
            _ => unreachable!(),
        };

        Self { name, val }
    }


    
}

#[derive(PartialEq, PartialOrd, Clone, Serialize, Deserialize)]
pub struct TempInvocation {
    pub rule: String,
    pub properties: Vec<TempProperty>,
}

impl TempInvocation {
    pub fn parse(invocation: &mut Pairs<Rule>) -> Self {
        let rule = invocation.next().unwrap().as_str().to_string();

        let properties = invocation
            .map(|p| TempProperty::parse(&mut p.into_inner()))
            .collect_vec();

        Self { rule, properties }
    }

    pub fn to_node(&self, parent_properties: &Properties, grammar: &Grammar) -> Node {
        Node {
            invocation: self.clone(),
            absolute_properties: parent_properties.make_absolute(&self.get_properties(grammar)),
            children: None,
        }
    }

    pub fn get_children(&self, absolute_properties: &Properties,  grammar: &Grammar)-> Vec<Node> {
            match self.rule.to_ascii_lowercase().as_str() {
            "circle" => Default::default(),// Command::Circle,
            "square" => Default::default(),
            x=> grammar.rules.get(x).unwrap().children.iter().map(|c|c.to_node(&absolute_properties, grammar)).collect_vec()
            
        }


    }

    fn get_properties(&self, grammar: &Grammar)-> Properties{
        let mut properties = Properties::default_additive();

        for prop in self.properties.iter(){
            let value = prop.val.try_get_value(&grammar.defs).unwrap();
            match prop.name.to_ascii_lowercase().as_str() {
                "p" => properties.p = value,
                "x" => properties.x = value,
                "y" => properties.y = value,
                "r" => properties.r = value,
                "h" => properties.h = value,
                "s" => properties.s = value,
                "l" => properties.l = value,
                x=> ()// return Err(format!("Property '{}' not defined", x)).unwrap()
            }
        }
        properties
    }

    // pub fn try_into(self, rules: &BTreeMap<String, Rc<UserRule>>, defs: &BTreeMap<String, f32>) -> Result<Invocation, String>{
    //     let command = 
        
    //     match self.rule.to_ascii_lowercase().as_str() {
    //         "circle" => Command::Circle,
    //         "square" => Command::Square,
    //         x=> Command::User { rule: rules.get(x).ok_or(format!("Rule '{}' not defined", self.rule))?.clone() }     
    //     };

    //     let mut properties = Properties::default();

    //     for prop in self.properties{
    //         let value = prop.val.try_get_value(defs)?;
    //         match prop.name.to_ascii_lowercase().as_str() {
    //             "p" => properties.p = value,
    //             "x" => properties.x = value,
    //             "y" => properties.y = value,
    //             "r" => properties.r = value,
    //             "h" => properties.h = value,
    //             "s" => properties.s = value,
    //             "l" => properties.l = value,
    //             x=> return Err(format!("Property '{}' not defined", x))
    //         }
    //     }

    //     Ok(Invocation{command, properties})


    // }   
}

#[derive(PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct Grammar {
    pub top_level: Vec<TempInvocation>,
    pub defs: BTreeMap<String, f32>,
    pub rules: BTreeMap<String, UserRule>,
}

impl Grammar {
    pub fn to_root_node(&self, settings: ExpandSettings) -> Node {
        let mut current = ExpandStatistics::default();
        let nodes = self
            .top_level
            .iter()
            .map(|i| i.to_node(&Properties::default_initial(), self))
            .collect_vec();

        let mut root =  Node {
            invocation: TempInvocation {
                rule: "root".to_string(),
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

// #[derive(PartialEq, PartialOrd, Clone)]
// pub enum Command {
//     Root,
//     Circle,
//     Square,
//     User { rule: Rc<UserRule> },
// }

#[derive(PartialEq, PartialOrd, Clone, Serialize, Deserialize)]
pub struct UserRule {
    pub name: String,
    pub children: Vec<TempInvocation>,
}

// #[derive(PartialEq, PartialOrd, Clone)]
// pub struct Invocation {
//     pub command: Command,
//     pub properties: Properties,
// }

// impl Invocation {
//     pub fn to_node(&self, parent_properties: &Properties) -> Node {
//         Node {
//             invocation: self.clone(),
//             absolute_properties: parent_properties.make_absolute(&self.properties),
//             children: None,
//         }
//     }
// }

#[derive(PartialEq, PartialOrd, Clone)]
pub struct Node {
    pub invocation: TempInvocation,
    pub absolute_properties: Properties,
    pub children: Option<Vec<Node>>,
}

impl Node {

    pub fn to_svg(&self, grammar: &Grammar) -> String{
         let elements = self.to_svg_element(grammar);

         format!("<svg viewbox=\"-1 -1 2 2\" width=\"100%\" height=\"100%\" > {} </svg>", elements)
    }

    pub fn to_svg_element(&self, grammar: &Grammar) -> String {
        let relative_properties = self.invocation.get_properties(grammar);

        if self.children.is_some() && !self.children.as_ref().unwrap().is_empty()    {
            let child_text = self.children.as_ref().unwrap().iter().map(|c| c.to_svg_element(grammar)).join("\r\n");


            format!("<g style=\"transform:  translate({x}px, {y}px) scale({p}%) rotate({r}deg);\">\r\n {child_text}\r\n </g>",

            x= relative_properties.x,
            y =   relative_properties.y,
            r = relative_properties.r,
            p =  relative_properties.p * 100.0,
                    //no color
            
            child_text = child_text)
            
        }
        else{
            //let absolute_properties = self.absolute_properties;//.make_absolute(&properties);
            
            match self.invocation.rule.as_str(){
                "circle"=>{
                    format!("<circle cx={x} cy={y} r={p} fill=\"hsl({h}, {s}%, {l}%)\" stroke=\"none\"   />", 
                    x= relative_properties.x,
                    y =  relative_properties.y,
                    //ignore rotation
                    p = relative_properties.p,
                    h = self.absolute_properties.h ,
                    s = self.absolute_properties.s * 100.0 ,
                    l = self.absolute_properties.l  * 100.0,                
                )
                },
                "square"=>{
                    format!("<rect x={x} y={y} width={p} height={p} fill=\"hsl({h}, {s}%, {l}%)\" stroke=\"none\" style=\"transform: rotate({r}deg);\" />", 
                    x= relative_properties.x -( relative_properties.p) ,
                    y =  relative_properties.y -(relative_properties.p) ,
                    r = relative_properties.r,
                    p =  relative_properties.p  * 2.0,
                    h = self.absolute_properties.h ,
                    s = self.absolute_properties.s * 100.0 ,
                    l = self.absolute_properties.l  * 100.0,                
                )
                }
                _=> "".to_string()
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
            let new_children =  self.invocation.get_children(&self.absolute_properties, grammar) 
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
        } else if node.absolute_properties.y.abs() - node.absolute_properties.p > 1.5 {
            true
        } else {
            false
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

#[derive(PartialEq, PartialOrd, Clone)]
pub struct Properties {
    p: f32,
    x: f32,
    y: f32,
    r: f32,
    h: f32,
    s: f32,
    l: f32,
    a: f32,
    d: usize,
}

impl Properties {
    ///Make absolute child properties from the child relative propeties
    pub fn make_absolute(&self, child: &Self) -> Self {
        let x2 = self.p
            * ((self.r.to_radians().cos() * child.x) - (self.r.to_radians().sin() * child.y));
        let y2 = self.p
            * ((self.r.to_radians().sin() * child.x) + (self.r.to_radians().cos() * child.y));

        Self {
            p: (self.p * child.p).clamp(0.0, 1.0),
            x: self.x + x2,
            y: self.y + y2,
            r: (((self.r + child.r) % 360.0) + 360.0) % 360.0,
            h: (((self.h + child.h) % 360.0) + 360.0) % 360.0,
            s: (self.s + child.s).clamp(0.0, 1.0),
            l: (self.l + child.l).clamp(0.0, 1.0),
            a: (self.a * child.a).clamp(0.0, 1.0),
            d: self.d + child.d,
        }
    }

    fn default_initial() -> Self {
        Self {
            p: 1.0,
            x: Default::default(),
            y: Default::default(),
            r: Default::default(),
            h: Default::default(),
            s: 1.0,
            l: 0.0,
            a: 1.0,
            d: Default::default(),
        }
    }

    fn default_additive() -> Self {
        Self {
            p: 1.0,
            x: Default::default(),
            y: Default::default(),
            r: Default::default(),
            h: Default::default(),
            s: 0.0,
            l: 0.0,
            a: 1.0,
            d: 1,
        }
    }
}

