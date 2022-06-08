use std::{collections::BTreeMap, default};

use crate::core::prelude::*;
use itertools::Itertools;
use num::traits::ops::inv;
use pest::iterators::Pairs;
use pest::Parser;
use pest_derive::Parser;
use serde::{Deserialize, Serialize};


#[derive(PartialEq, PartialOrd, Clone, Serialize, Deserialize)]
pub struct TempProperties {
    p: TempValue,
    x: TempValue,
    y: TempValue,
    r: TempValue,
    h: TempValue,
    s: TempValue,
    v: TempValue,
    a: TempValue,
}

impl TempProperties {
    pub fn try_convert(&self, values: &BTreeMap<String, f32>) -> Result<Properties, String> {
        let p = self.p.try_get_value(values)?;
        let x = self.x.try_get_value(values)?;
        let y = self.y.try_get_value(values)?;
        let r = self.r.try_get_value(values)?;
        let h = self.h.try_get_value(values)?;
        let s = self.s.try_get_value(values)?;
        let v = self.v.try_get_value(values)?;
        let a = self.a.try_get_value(values)?;

        Ok(Properties {
            p,
            x,
            y,
            r,
            h,
            s,
            v,
            a,
            d: 1,
        })
    }
}

impl TryFrom<Vec<TempProperty>> for TempProperties {
    type Error = String;

    fn try_from(vector: Vec<TempProperty>) -> Result<Self, Self::Error> {
        let mut properties = TempProperties::default();

        for prop in vector {
            match prop.name.to_ascii_lowercase().as_str() {
                "p" => properties.p = prop.val,
                "x" => properties.x = prop.val,
                "y" => properties.y = prop.val,
                "r" => properties.r = prop.val,
                "h" => properties.h = prop.val,
                "s" => properties.s = prop.val,
                "v" => properties.v = prop.val,
                x => return Err(format!("Property '{}' not defined", x)).unwrap(),
            }
        }
        Ok(properties)
    }
}

impl Default for TempProperties {
    fn default() -> Self {
        Self {
            p: TempValue::Number { val: 1.0 },
            x: TempValue::Number { val: 0.0 },
            y: TempValue::Number { val: 0.0 },
            r: TempValue::Number { val: 0.0 },
            h: TempValue::Number { val: 0.0 },
            s: TempValue::Number { val: 0.0 },
            v: TempValue::Number { val: 0.0 },
            a: TempValue::Number { val: 1.0 },
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
        let name = property.next().unwrap().as_str().to_ascii_lowercase();

        let next = property.next().unwrap().into_inner().next().unwrap();

        let rule = next.as_rule();

        let val = match rule {
            Rule::number => {
                let val_string: String = next
                    .as_str()
                    .chars()
                    .map(|c| match c {
                        'm' => '-',
                        'M' => '-',
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


#[derive(PartialEq, PartialOrd, Clone)]
pub struct Properties {
    pub p: f32,
    pub x: f32,
    pub y: f32,
    pub r: f32,
    pub h: f32,
    pub s: f32,
    pub v: f32,
    pub a: f32,
    pub d: usize,
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
            v: (self.v + child.v).clamp(0.0, 1.0),
            a: (self.a * child.a).clamp(0.0, 1.0),
            d: self.d + child.d,
        }
    }

    pub fn default_initial() -> Self {
        Self {
            p: 1.0,
            x: Default::default(),
            y: Default::default(),
            r: Default::default(),
            h: Default::default(),
            s: 1.0,
            v: 0.0,
            a: 1.0,
            d: Default::default(),
        }
    }

    pub fn default_additive() -> Self {
        Self {
            p: 1.0,
            x: Default::default(),
            y: Default::default(),
            r: Default::default(),
            h: Default::default(),
            s: 0.0,
            v: 0.0,
            a: 1.0,
            d: 1,
        }
    }
}