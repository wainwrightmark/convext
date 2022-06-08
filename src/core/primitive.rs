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
    pub fn to_svg(&self, relative_properties : &NodeProperties, absolute_properties: &NodeProperties) -> String{

        let rotate_transform = if relative_properties.r == 0.0 {"".to_string()} else{format!("style=\"transform: rotate({r}deg);\"", r= relative_properties.r)};

        match self {
            Primitive::Circle => format!("<ellipse cx={x} cy={y} rx={rx} ry={ry} fill=\"hsl({h}, {s}%, {l}%)\" stroke=\"none\"  {rotate_transform} />", 
            x= relative_properties.x,
            y =  relative_properties.y,
            //ignore rotation
            rx = relative_properties.p * absolute_properties.w,
            ry = relative_properties.p * absolute_properties.l,
            h = absolute_properties.h ,
            s = absolute_properties.s * 100.0 ,
            l = absolute_properties.v  * 100.0,                
            rotate_transform = rotate_transform
        ),
            Primitive::Square => format!("<rect x={x} y={y} width={width} height={height} fill=\"hsl({h}, {s}%, {l}%)\" stroke=\"none\" {rotate_transform} />", 
            x= relative_properties.x -( relative_properties.p) ,
            y =  relative_properties.y -(relative_properties.p) ,            
            width =  relative_properties.p *absolute_properties.w * 2.0,
            height =  relative_properties.p *absolute_properties.l * 2.0,
            h = absolute_properties.h ,
            s = absolute_properties.s * 100.0 ,
            l = absolute_properties.v  * 100.0,                
            rotate_transform = rotate_transform
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