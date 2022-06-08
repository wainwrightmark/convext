use std::{collections::BTreeMap, default};

use crate::core::prelude::*;
use itertools::Itertools;
use num::traits::ops::inv;
use pest::iterators::Pairs;
use pest::Parser;
use pest_derive::Parser;
use serde::{Deserialize, Serialize};


#[derive(PartialEq, PartialOrd, Copy, Clone, Serialize, Deserialize)]
pub enum Primitive{
    Circle,
    Square,
    //TODO others
}

impl Primitive {
    pub fn to_svg(&self, relative_properties : &Properties, absolute_properties: &Properties) -> String{
        match self {
            Primitive::Circle => format!("<circle cx={x} cy={y} r={p} fill=\"hsl({h}, {s}%, {l}%)\" stroke=\"none\"   />", 
            x= relative_properties.x,
            y =  relative_properties.y,
            //ignore rotation
            p = relative_properties.p,
            h = absolute_properties.h ,
            s = absolute_properties.s * 100.0 ,
            l = absolute_properties.v  * 100.0,                
        ),
            Primitive::Square => format!("<rect x={x} y={y} width={p} height={p} fill=\"hsl({h}, {s}%, {l}%)\" stroke=\"none\" style=\"transform: rotate({r}deg);\" />", 
            x= relative_properties.x -( relative_properties.p) ,
            y =  relative_properties.y -(relative_properties.p) ,
            r = relative_properties.r,
            p =  relative_properties.p  * 2.0,
            h = absolute_properties.h ,
            s = absolute_properties.s * 100.0 ,
            l = absolute_properties.v  * 100.0,                
        ),
        }
    }
}

impl std::str::FromStr for Primitive{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "circle"=> Ok(Primitive::Circle),
            "square"=> Ok(Primitive::Square),
            _=> Err("Could not parse".to_string())
        }
    }
}