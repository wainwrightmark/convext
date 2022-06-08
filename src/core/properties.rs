use std::{collections::BTreeMap, default, str::FromStr};

use crate::core::prelude::*;
use itertools::Itertools;
use num::traits::ops::inv;
use pest::iterators::Pairs;
use pest::Parser;
use pest_derive::Parser;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, PartialOrd, Clone, Serialize, Deserialize)]
pub enum Value {
    Number { val: f32 },
    Variable { name: String },
}

impl Value {
    pub fn try_get_value(&self, defs: &BTreeMap<String, f32>) -> Result<f32, String> {
        match self {
            Value::Number { val } => Ok(*val),
            Value::Variable { name } => defs
                .get(&name.to_ascii_lowercase())
                .ok_or(format!("Varaible '{}' not defined", name))
                .map(|&x| x),
        }
    }
}

#[derive(PartialEq, PartialOrd, Clone, Serialize, Deserialize)]
pub struct TempProperties {
    p: Value,
    x: Value,
    y: Value,
    r: Value,
    h: Value,
    s: Value,
    v: Value,
    a: Value,
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

impl Default for TempProperties {
    fn default() -> Self {
        Self {
            p: Value::Number { val: 1.0 },
            x: Value::Number { val: 0.0 },
            y: Value::Number { val: 0.0 },
            r: Value::Number { val: 0.0 },
            h: Value::Number { val: 0.0 },
            s: Value::Number { val: 0.0 },
            v: Value::Number { val: 0.0 },
            a: Value::Number { val: 1.0 },
        }
    }
}

#[derive(PartialEq, PartialOrd, Clone, Copy, Serialize, Deserialize)]
pub enum PropertyType {
    AnyPositive,
    Any,
    UnitInterval,
    Degrees,
}

impl PropertyType {
    ///Deconstruct this into min, max, step
    pub fn deconstruct(self) -> (f32, f32, f32) {
        match self {
            PropertyType::UnitInterval => (0.0, 1.0, 0.05),
            PropertyType::AnyPositive => (0.0, 2.0, 0.05),
            PropertyType::Any => (-2.0, 2.0, 0.05),
            PropertyType::Degrees => (0.0, 360.0, 15.0),
        }
    }
}

#[derive(PartialEq, PartialOrd, Clone, Copy, Serialize, Deserialize)]
pub enum PropertyKey {
    P,
    X,
    Y,
    R,

    H,
    S,
    V,
    A,
}

impl PropertyKey {
    pub fn set(self, properties: &mut TempProperties, value: Value) {
        match self {
            PropertyKey::P => properties.p = value,
            PropertyKey::X => properties.x = value,
            PropertyKey::Y => properties.y = value,
            PropertyKey::R => properties.r = value,
            PropertyKey::H => properties.h = value,
            PropertyKey::S => properties.s = value,
            PropertyKey::V => properties.v = value,
            PropertyKey::A => properties.a = value,
        }
    }

    pub fn get_type(self)-> PropertyType{
        match self{
            PropertyKey::P => PropertyType::AnyPositive,
            PropertyKey::X => PropertyType::Any,
            PropertyKey::Y => PropertyType::Any,
            PropertyKey::R => PropertyType::Degrees,
            PropertyKey::H => PropertyType::Degrees,
            PropertyKey::S => PropertyType::UnitInterval,
            PropertyKey::V => PropertyType::UnitInterval,
            PropertyKey::A => PropertyType::UnitInterval,
        }
    }
}

impl std::str::FromStr for PropertyKey {
    type Err = String;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        match name.to_ascii_lowercase().as_str() {
            "p" => Ok(PropertyKey::P),
            "x" => Ok(PropertyKey::X),
            "y" => Ok(PropertyKey::Y),
            "r" => Ok(PropertyKey::R),
            "h" => Ok(PropertyKey::H),
            "s" => Ok(PropertyKey::S),
            "v" => Ok(PropertyKey::V),
            "a" => Ok(PropertyKey::A),
            x => return Err(format!("Property '{}' not defined", x)).unwrap(),
        }
    }
}

#[derive(PartialEq, PartialOrd, Clone, Serialize, Deserialize)]
pub struct TempProperty {
    pub key: PropertyKey,
    pub value: Value,
}

impl TempProperty {
    pub fn try_parse(property: &mut Pairs<Rule>) -> Result<Self, String> {
        let name = property.next().unwrap().as_str();
        let key = PropertyKey::from_str(name)?;

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
                Value::Number { val }
            }
            Rule::variable => {
                let name = next.as_str().replacen('?', "", 1);
                Value::Variable { name }
            }
            _ => unreachable!(),
        };

        Ok(Self { key, value: val })
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
