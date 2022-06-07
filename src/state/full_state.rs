use crate::core::prelude::*;
use crate::state::prelude::*;
use itertools::Itertools;
use num::ToPrimitive;
use serde::*;
use std::default;
use std::rc::Rc;
use yewdux::prelude::*;

#[derive(PartialEq,  Store, Clone, Serialize, Deserialize)]
#[store(storage = "local")] // can also be "session"
pub struct InputState {
    pub text: String,
    pub settings: ExpandSettings
}

impl Default for InputState{
    fn default() -> Self {
        Self { text: "circle".to_string(), settings: Default::default() }
    }
}

impl InputState{

    pub fn update_text(&mut self, new_text: String) {
        if self.text != new_text{
            self.text = new_text.clone();
            Dispatch::<ImageState>::new().reduce_mut(|state: &mut ImageState|state.maybe_update_text(new_text, self.settings) );
        }
    }

    pub fn get_svg_text(&self) -> Result<String, String>{
        let grammar =parse(self.text.as_str())?;

        let node = grammar.to_root_node(self.settings);
    
        let svg = node.to_svg(&grammar);

        Ok(svg)
    }
}


#[derive(PartialEq,  Store, Clone, Serialize, Deserialize, Default)]
pub struct ImageState{
    pub grammar: Grammar,
    pub svg: String,
    pub error: Option<String>
}

impl ImageState{

    pub fn update_settings(&mut self, settings : ExpandSettings){
        let node  = self.grammar.to_root_node(settings);
        let svg = node.to_svg(&self.grammar);
        
        self.svg = svg;
    }

    pub fn maybe_update_text(&mut self, text: String, settings : ExpandSettings){
        let grammar_result =parse(text.as_str());

        match grammar_result {
            Ok(grammar) => {
                self.error = None;
                if self.grammar != grammar{
                    
                    let node  = grammar.to_root_node(settings);
                    let svg = node.to_svg(&grammar);

                    self.grammar = grammar;
                    self.svg = svg;
                }
            },
            Err(error) => self.error = Some(error),
        }
    }
}
