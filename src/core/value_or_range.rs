use std::ops::{Add, Mul, Sub};
use std::{collections::BTreeMap, default, str::FromStr};
use crate::core::prelude::*;
use itertools::Itertools;
use num::traits::ops::inv;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;
use rand::Rng;
use rand::prelude::StdRng;
use serde::{Deserialize, Serialize};


#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub enum ValueOrRange{
    Value(f32),
    Range{start: f32, end: f32}    
}

impl Add for ValueOrRange{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match self{
            ValueOrRange::Value(v) => rhs.apply(|x| x +v),
            ValueOrRange::Range { start, end } => match rhs {
                ValueOrRange::Value(v) => Self::Range { start: start + v, end: end + v },
                ValueOrRange::Range { start:s2, end:e2 } => Self::Range { start: start + s2, end: end + e2 },
            },
        }
    }
}

impl Sub for ValueOrRange{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match self{
            ValueOrRange::Value(v) => rhs.apply(|x| v - x),
            ValueOrRange::Range { start, end } => match rhs {
                ValueOrRange::Value(v) => Self::Range { start: start - v, end: end - v },
                ValueOrRange::Range { start:s2, end:e2 } => Self::Range { start: start - s2, end: end - e2 },
            },
        }
    }
}

impl Mul for ValueOrRange{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match self{
            ValueOrRange::Value(v) => rhs.apply(|x| x * v),
            ValueOrRange::Range { start, end } => match rhs {
                ValueOrRange::Value(v) => Self::Range { start: start * v, end: end * v },
                ValueOrRange::Range { start:s2, end:e2 } => Self::Range { start: start * s2, end: end * e2 },
            },
        }
    }
}

impl Default for ValueOrRange{
    fn default() -> Self {
        ValueOrRange::Value(0.0)
    }
}

impl From<f32> for ValueOrRange{
    fn from(v: f32) -> Self {
        ValueOrRange::Value(v)
    }
}

impl ValueOrRange{

    pub fn random_value(self, rng: &mut StdRng)->f32{
        match self{
            ValueOrRange::Value(v) => v,
            ValueOrRange::Range { start, end } => 
            if start<=end{rng.gen_range(start..=end)} else{rng.gen_range(end..=start)}
            ,
        }
    }

    pub fn max_abs(self)-> f32{
        match self{
            ValueOrRange::Value(v) => v.abs(),
            ValueOrRange::Range { start, end } => start.abs().max(end.abs()),
        }
    }
    
    pub fn min_abs(self)-> f32{
        match self{
            ValueOrRange::Value(v) => v.abs(),
            ValueOrRange::Range { start, end } => start.abs().min(end.abs()),
        }
    }

    fn apply(self, f: impl Fn(f32) -> f32)-> Self{
        match self{
            ValueOrRange::Value(v) => f(v).into(),
            ValueOrRange::Range { start, end } => ValueOrRange::Range { start:f(start), end:f(end) },
        }
    }

    pub fn max(self, other: f32)-> Self{
        self.apply(|x|x.max(other))
    }
    
    pub fn min(self, other: f32)-> Self{
        self.apply(|x|x.min(other))
    }
    
    pub fn clamp(self, min: f32, max: f32)-> Self{
        self.apply(|x|x.clamp(min, max))
    }

    pub fn cos_degrees(self)-> Self{
        self.apply(|x|x.to_radians().cos())
    }
    
    pub fn sin_degrees(self)-> Self{
        self.apply(|x|x.to_radians().sin())
    }

    pub fn mod360(self)-> Self{
        self.apply(|x|((x % 360.0)+ 360.0) % 360.0)
    }

    pub fn min_value(self)-> f32{
        match self{
            ValueOrRange::Value(v) => v,
            ValueOrRange::Range { start, end } => start.min(end),
        }
    }
    
    pub fn max_value(self)-> f32{
        match self{
            ValueOrRange::Value(v) => v,
            ValueOrRange::Range { start, end } => start.max(end),
        }
    }


}