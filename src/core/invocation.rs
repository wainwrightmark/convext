use std::{collections::BTreeMap, default, str::FromStr};

use crate::core::prelude::*;
use itertools::Itertools;
use num::traits::ops::inv;
use pest::iterators::Pairs;
use pest::Parser;
use pest_derive::Parser;
use rand::{prelude::StdRng, SeedableRng, Rng};
use serde::{Deserialize, Serialize};
use yew::Properties;

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
        rng: &mut StdRng,
    ) -> Vec<Node> {
        match self.method.clone() {
            Method::Primitive(_) => Default::default(),
            Method::Root => unreachable!(),
            Method::Rule(r) =>{
                let mut rng1 = StdRng::from_seed(rng.gen());
                let mut rng2 = StdRng::from_seed(rng.gen());

                grammar
                .rules
                .get(&r)
                .unwrap()
                .cases
                .iter()
                .filter(|&c| c.should_enter(grammar, absolute_properties, &mut rng1))
                .take(1) //only take the first condition which matches
                .flat_map(|c| c.invocations.iter())
                .map(|c| c.to_node(absolute_properties, grammar, &mut rng2))
                .collect_vec()


            }
        }
    }

    pub fn to_node(&self, parent_properties: &NodeProperties, grammar: &Grammar, rng: &mut StdRng,) -> Node {
        Node {
            invocation: self.clone(),
            absolute_properties: parent_properties.make_absolute(&NodeProperties::from_temp(
                &self.properties,
                grammar,
                parent_properties,
                rng
            )),
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
