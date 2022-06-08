use crate::core::prelude::*;
use crate::state::prelude::*;
use itertools::Itertools;
use num::ToPrimitive;
use serde::*;
use std::collections::BTreeMap;
use std::default;
use std::rc::Rc;
use yewdux::prelude::*;

#[derive(PartialEq, Store, Clone, Serialize, Deserialize)]
#[store(storage = "local")] // can also be "session"
pub struct InputState {
    pub text: String,
    pub grammar: Grammar,
    pub overrides: BTreeMap<String, f32>,
    pub settings: ExpandSettings,
    pub error: Option<String>,
}

impl Default for InputState {
    fn default() -> Self {
        let text = "circle v 0.5\r\ncircle p 0.5 v 0.5 h 120";
        let grammar = parse(text).unwrap();

        Self {
            text: text.to_string(),
            grammar,
            overrides: Default::default(),
            settings: Default::default(),
            error: Default::default(),
        }
    }
}

impl InputState {
    pub fn get_variable_value(&self, key: &String) -> f32 {
        if let (Some(v)) = self.overrides.get(key) {
            v.clone()
        } else if let Some(v) = self.grammar.defs.get(key) {
            v.clone()
        } else {
            0.0
        }
    }

    pub fn set_variable_value(&mut self, key: String, value: f32) {
        self.overrides.insert(key, value);
        Dispatch::<ImageState>::new().reduce_mut(|state: &mut ImageState| state.update_svg(&self));
    }

    pub fn update_settings(&mut self, settings: ExpandSettings) {
        let node = self.grammar.expand(&settings);
        let svg = node.to_svg(&self.grammar);

        self.settings = settings;
        Dispatch::<ImageState>::new().reduce_mut(|state: &mut ImageState| state.update_svg(&self));
    }

    pub fn update_text(&mut self, new_text: String) {
        if self.text != new_text {
            self.text = new_text.clone();
            let grammar_result = parse(new_text.as_str());

            match grammar_result {
                Ok(grammar) => {
                    self.error = None;
                    if self.grammar != grammar {
                        self.grammar = grammar;
                        Dispatch::<ImageState>::new()
                            .reduce_mut(|state: &mut ImageState| state.update_svg(&self));
                    }
                }
                Err(error) => self.error = Some(error),
            }
        }
    }
}
