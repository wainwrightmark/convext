use std::{collections::BTreeMap, default, str::FromStr};
use crate::core::prelude::*;
use itertools::Itertools;
use num::traits::ops::inv;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;
use rand::prelude::StdRng;
use serde::{Deserialize, Serialize};


#[derive(PartialEq, PartialOrd, Clone)]
pub struct NodeProperties {
    pub p: ValueOrRange,
    pub l: ValueOrRange,
    pub w: ValueOrRange,
    pub c: ValueOrRange,
    pub x: ValueOrRange,
    pub y: ValueOrRange,
    pub r: ValueOrRange,
    pub h: ValueOrRange,
    pub s: ValueOrRange,
    pub v: ValueOrRange,
    pub a: ValueOrRange,
    pub d: usize,
}



impl NodeProperties {
    pub fn from_temp(
        vector: &Vec<TempProperty>,
        grammar: &Grammar,
        context: &NodeProperties,
        rng: &mut StdRng,
    ) -> Self {
        let mut properties = Self::default_additive();

        for prop in vector {
            let value = prop.value.try_get_value(grammar, context, rng).unwrap();
            prop.key.set(&mut properties, value);
        }

        properties
    }

    ///Make absolute child properties from the child relative propeties
    pub fn make_absolute(&self, child: &Self) -> Self {
        let x2 = self.p
            * ((self.r.cos_degrees() * child.x) - (self.r.sin_degrees() * child.y));
        let y2 = self.p
            * ((self.r.sin_degrees() * child.x) + (self.r.cos_degrees() * child.y));

        Self {
            p: (self.p * child.p).max(0.0),
            l: self.l * child.l.max(0.0),
            w: self.w * child.w.max(0.0),
            c: self.c + child.c.clamp(0.0, 1.0),
            x: self.x + x2,
            y: self.y + y2,
            r: (self.r + child.r).mod360(),
            h: (self.h + child.h).mod360(),
            s: (self.s + child.s).clamp(0.0, 1.0),
            v: (self.v + child.v).clamp(0.0, 1.0),
            a: (self.a * child.a).clamp(0.0, 1.0),
            d: self.d + child.d,
        }
    }

    pub fn default_initial() -> Self {
        Self {
            p: 1.0.into(),
            l: 1.0.into(),
            w: 1.0.into(),
            c: Default::default(),
            x: Default::default(),
            y: Default::default(),
            r: Default::default(),
            h: Default::default(),
            s: 1.0.into(),
            v: 0.0.into(),
            a: 1.0.into(),
            d: Default::default(),
        }
    }

    pub fn default_additive() -> Self {
        Self {
            p: 1.0.into(),
            l: 1.0.into(),
            w: 1.0.into(),
            c: Default::default(),
            x: Default::default(),
            y: Default::default(),
            r: Default::default(),
            h: Default::default(),
            s: 0.0.into(),
            v: 0.0.into(),
            a: 1.0.into(),
            d: 1,
        }
    }
}