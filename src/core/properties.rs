use std::{collections::BTreeMap, default, str::FromStr};

use crate::core::prelude::*;
use itertools::Itertools;
use num::traits::ops::inv;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;
use serde::{Deserialize, Serialize};


#[derive(PartialEq, PartialOrd, Clone, Serialize, Deserialize)]
pub struct TempProperty {
    pub key: PropertyKey,
    pub value: ExpressionOrRange,
}

impl TempProperty {
    pub fn try_parse(property: &mut Pairs<Rule>) -> Result<Self, String> {
        let name = property.next().unwrap().as_str();
        let key = PropertyKey::from_str(name)?;

        let next = property.next().unwrap().into_inner().next().unwrap();

        let value = ExpressionOrRange::parse(next)?;

        Ok(Self { key, value })
    }    
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
pub enum PropertyType {
    AnyPositive,
    Any,
    UnitInterval,
    Degrees,
    IntegerPositive,
    Boolean
}

impl PropertyType {
    ///Deconstruct this into min, max, step
    pub fn deconstruct(self) -> (f32, f32, f32) {
        match self {
            PropertyType::UnitInterval => (0.0, 1.0, 0.05),
            PropertyType::AnyPositive => (0.0, 2.0, 0.05),
            PropertyType::Any => (-2.0, 2.0, 0.05),
            PropertyType::Degrees => (0.0, 360.0, 5.0),
            PropertyType::Boolean => (0.0,1.0,1.0),
            PropertyType::IntegerPositive => (0.0,1000.0,1.0),
        }
    }
}

#[derive(PartialEq, PartialOrd, Clone, Copy, Serialize, Deserialize)]
pub enum PropertyKey {
    P,
    L,
    W,
    C,

    X,
    Y,
    R,

    H,
    S,
    V,
    A,

    D
}

impl PropertyKey {
    pub fn set(self, properties: &mut NodeProperties, value: ValueOrRange) {
        match self {
            PropertyKey::P => properties.p = value,
            PropertyKey::L => properties.l = value,
            PropertyKey::W => properties.w = value,
            PropertyKey::C => properties.c = value,
            PropertyKey::X => properties.x = value,
            PropertyKey::Y => properties.y = value,
            PropertyKey::R => properties.r = value,
            PropertyKey::H => properties.h = value,
            PropertyKey::S => properties.s = value,
            PropertyKey::V => properties.v = value,
            PropertyKey::A => properties.a = value,
            PropertyKey::D => properties.d = match value {
                ValueOrRange::Value(v) => v.round() as usize,
                ValueOrRange::Range { start, end } => start.round() as usize,
            }
        }
    }

    pub fn get(self, properties: &NodeProperties) -> ValueOrRange {
        match self {
            PropertyKey::P => properties.p,
            PropertyKey::L => properties.l,
            PropertyKey::W => properties.w,
            PropertyKey::C => properties.c,
            PropertyKey::X => properties.x,
            PropertyKey::Y => properties.y,
            PropertyKey::R => properties.r,
            PropertyKey::H => properties.h,
            PropertyKey::S => properties.s,
            PropertyKey::V => properties.v,
            PropertyKey::A => properties.a,
            PropertyKey::D => ValueOrRange::Value(properties.d as f32,) 
        }
    }

    pub fn get_type(self) -> PropertyType {
        match self {
            PropertyKey::P => PropertyType::AnyPositive,
            PropertyKey::L => PropertyType::AnyPositive,
            PropertyKey::W => PropertyType::AnyPositive,
            PropertyKey::C => PropertyType::UnitInterval,
            PropertyKey::X => PropertyType::Any,
            PropertyKey::Y => PropertyType::Any,
            PropertyKey::R => PropertyType::Degrees,
            PropertyKey::H => PropertyType::Degrees,
            PropertyKey::S => PropertyType::UnitInterval,
            PropertyKey::V => PropertyType::UnitInterval,
            PropertyKey::A => PropertyType::UnitInterval,
            PropertyKey::D => PropertyType::IntegerPositive,
        }
    }
}

impl std::str::FromStr for PropertyKey {
    type Err = String;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        match name.to_ascii_lowercase().as_str() {
            "p" => Ok(PropertyKey::P),
            "l" => Ok(PropertyKey::L),
            "w" => Ok(PropertyKey::W),
            "c" => Ok(PropertyKey::C),
            "x" => Ok(PropertyKey::X),
            "y" => Ok(PropertyKey::Y),
            "r" => Ok(PropertyKey::R),
            "h" => Ok(PropertyKey::H),
            "s" => Ok(PropertyKey::S),
            "v" => Ok(PropertyKey::V),
            "a" => Ok(PropertyKey::A),
            "d" => Ok(PropertyKey::D),
            x => return Err(format!("Property '{}' not defined", x)).unwrap(),
        }
    }
}




