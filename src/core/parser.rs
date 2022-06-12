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

#[derive(Parser)]
#[grammar = "core/convext.pest"]
pub struct ConvextParser;

pub fn parse(input: &str) -> Result<Grammar, String> {
    let mut file_pairs = ConvextParser::parse(Rule::file, input).map_err(|e| e.to_string())?;
    let file = file_pairs.next().unwrap();

    let mut defs = BTreeMap::<String, f32>::default();
    let mut rules = BTreeMap::<String, UserRule>::default();

    let mut top_level = Vec::<Invocation>::default();

    for pair in file.into_inner() {
        match pair.as_rule() {
            Rule::statement => {
                let statement = pair.into_inner().next().unwrap();

                match statement.as_rule() {
                    Rule::invocation => {
                        let ii = Invocation::try_parse(&mut statement.into_inner())?;
                        top_level.push(ii);
                    }
                    Rule::rule => {
                        let mut inner = statement.into_inner();
                        let rule_keyword = inner.next();
                        let name = inner.next().unwrap().as_str().to_string();

                        let mut invocations = Vec::<Invocation>::new();


                        let mut probability: Option<Expression> = None;

                        for p in inner {
                            match p.as_rule() {
                                Rule::EOI => (),
                                Rule::expression => {
                                    probability = Some(Expression::parse(p.into_inner().next().unwrap())?) ;
                                }
                                Rule::keyword_end => (),
                                Rule::invocation => {
                                    let ti = Invocation::try_parse(&mut p.into_inner())?;
                                    invocations.push(ti);
                                }
                                _ => unreachable!(),
                            }
                        }

                        let key = name.to_ascii_lowercase();
                        let rule_case = RuleCase{
                            probability,
                            invocations,
                        };

                        if let Some(existing) = rules.get_mut(&key){

                            if existing.cases.iter().any(|c| c.probability.is_none()){
                                return Err(format!("Rule '{}' is defined after an unconditional rule of the same name", name));
                            }
                            existing.cases.push(rule_case);
                        }
                        else{

                            let new_rule = UserRule{
                                name: name.clone(),
                                cases: vec![rule_case]
                                
                            };

                            rules.insert(key, new_rule);
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

    for rule_name in top_level.iter().filter_map(|i| {
        if let Method::Rule(r) = i.method.clone() {
            Some(r)
        } else {
            None
        }
    }) {
        if !rules.contains_key(&rule_name) {
            return Err(format!("Rule '{}' does not exist", rule_name));
        }
    }

    for (_, user_rule) in rules.iter() {
        for rule_case in user_rule.cases.iter(){
            for rule_name in rule_case.invocations.iter().filter_map(|i| {
                if let Method::Rule(r) = i.method.clone() {
                    Some(r)
                } else {
                    None
                }
            }) {
                if !rules.contains_key(&rule_name) {
                    return Err(format!("Rule '{}' does not exist", rule_name));
                }
            }
        }
       
    }

    Ok(Grammar {
        defs,
        rules,
        top_level,
    })
}


