use crate::core::prelude::*;
use crate::state::prelude::*;
use itertools::Itertools;
use num::ToPrimitive;
use serde::*;
use std::default;
use std::rc::Rc;
use yewdux::prelude::*;


#[derive(PartialEq, Store, Clone, Serialize, Deserialize)]
pub struct ImageState {
    pub grammar: Grammar,
    pub svg: String,
    pub error: Option<String>,
}

impl Default for ImageState{
    fn default() -> Self {
        let mut s = Self { grammar: Default::default(), svg: Default::default(), error: Default::default() };

        let v = Dispatch::<InputState>::new().get();        
        s.maybe_update_text(v.as_ref().text.clone(), v.settings);

        s
    }
}

impl ImageState {
    pub fn update_settings(&mut self, settings: ExpandSettings) {
        let node = self.grammar.to_root_node(settings);
        let svg = node.to_svg(&self.grammar);

        self.svg = svg;
    }

    pub fn maybe_update_text(&mut self, text: String, settings: ExpandSettings) {
        let grammar_result = parse(text.as_str());

        match grammar_result {
            Ok(grammar) => {
                self.error = None;
                if self.grammar != grammar {
                    let node = grammar.to_root_node(settings);
                    let svg = node.to_svg(&grammar);

                    self.grammar = grammar;
                    self.svg = svg;
                }
            }
            Err(error) => self.error = Some(error),
        }
    }
}
