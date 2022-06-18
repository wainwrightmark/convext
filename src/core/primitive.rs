use std::{collections::BTreeMap, default};

use crate::core::prelude::*;
use itertools::Itertools;
use num::traits::ops::inv;
use pest::iterators::Pairs;
use pest::Parser;
use pest_derive::Parser;
use rand::prelude::StdRng;
use serde::{Deserialize, Serialize, __private::de};

#[derive(PartialEq, PartialOrd, Copy, Clone, Serialize, Deserialize)]
pub enum Primitive {
    Circle,
    Square,
    RightTriangle,
    Polygon(usize),
    //TODO others
}

impl Primitive {
    fn get_polygon_points(sides: usize) -> impl Iterator<Item = (f32, f32)> {
        (0..sides).map(move |side| {
            let degrees: f32 = (360.0f32 * side as f32 / sides as f32);
            let radians = degrees.to_radians();

            //First point is (0, 1)
            let x = -radians.sin();
            let y = -radians.cos();

            (x, y)
        })
    }

    pub fn to_svg(
        &self,
        relative_properties: &NodeProperties,
        absolute_properties: &NodeProperties, 
        rng: &mut StdRng,
        
    ) -> String {
        let rotate_transform = if relative_properties.r == 0.0.into() {
            "".to_string()
        } else {
            format!(
                "style=\"transform: rotate({r}deg);\"",
                r = relative_properties.r.random_value(rng)
            )
        };
        let color = format!(
            "fill=\"hsl({h}, {s}%, {l}%, {a}%)\" stroke=\"none\"",
            h = absolute_properties.h.random_value(rng),
            s = absolute_properties.s.random_value(rng) * 100.0,
            l = absolute_properties.v.random_value(rng) * 100.0,
            a = absolute_properties.a.random_value(rng) * 100.0,
        );

        match self {
            Primitive::Circle => format!(
                "<ellipse cx={x} cy={y} rx={rx} ry={ry} {color} {rotate_transform} />",
                x = relative_properties.x.random_value(rng),
                y = relative_properties.y.random_value(rng),
                //ignore rotation
                rx = relative_properties.p.random_value(rng) * absolute_properties.w.random_value(rng),
                ry = relative_properties.p.random_value(rng) * absolute_properties.l.random_value(rng),
                color = color,
                rotate_transform = rotate_transform
            ),
            Primitive::Square => {
                let x = relative_properties.x - (relative_properties.p * absolute_properties.w);
                let y = relative_properties.y - (relative_properties.p * absolute_properties.l);

                let width = relative_properties.p * absolute_properties.w * 2.0.into();
                let height = relative_properties.p * absolute_properties.l * 2.0.into();

                let rx = relative_properties.p * absolute_properties.c;
                let ry = relative_properties.p * absolute_properties.c;

                format!("<rect x={x} y={y} width={width} height={height} rx={rx} ry={ry} {color}  {rotate_transform} />", 
                x=x.random_value(rng),
                y=y.random_value(rng),
                rx = rx.random_value(rng),
                ry=ry.random_value(rng),
                width=width.random_value(rng),
                height=height.random_value(rng),
                color= color,
                rotate_transform = rotate_transform
            )
            }
            Primitive::RightTriangle => {
                let points = [(0.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]
                    .into_iter()
                    .flat_map(|(x, y)| {
                        [
                            (x * relative_properties.p.random_value(rng) * absolute_properties.w.random_value(rng))
                                + relative_properties.x.random_value(rng),
                            (y * relative_properties.p.random_value(rng) * absolute_properties.l.random_value(rng))
                                + relative_properties.y.random_value(rng),
                        ]
                    })
                    .join(" ");

                format!(
                    "<polygon points=\"{points}\" {color} {rotate_transform}/>",
                    rotate_transform = rotate_transform
                )
            }

            Primitive::Polygon(sides) => {
                let points = Self::get_polygon_points(*sides)
                    .flat_map(|(x, y)| {
                        [
                            (x * relative_properties.p.random_value(rng) * absolute_properties.w.random_value(rng))
                                + relative_properties.x.random_value(rng),
                            (y * relative_properties.p.random_value(rng) * absolute_properties.l.random_value(rng))
                                + relative_properties.y.random_value(rng),
                        ]
                    })
                    .join(" ");

                format!(
                    "<polygon points=\"{points}\" {color} {rotate_transform}/>",
                    rotate_transform = rotate_transform
                )
            }
        }
    }
}

impl std::str::FromStr for Primitive {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "circle" => Ok(Primitive::Circle),
            "square" => Ok(Primitive::Square),
            "rtriangle" => Ok(Primitive::RightTriangle),
            "triangle" => Ok(Primitive::Polygon(3)),
            "pentagon" => Ok(Primitive::Polygon(5)),
            "hexagon" => Ok(Primitive::Polygon(6)),
            "heptagon" => Ok(Primitive::Polygon(7)),
            "octagon" => Ok(Primitive::Polygon(8)),
            "nonagon" => Ok(Primitive::Polygon(9)),
            "decagon" => Ok(Primitive::Polygon(10)),
            "undecagon" => Ok(Primitive::Polygon(11)),
            "dodecagon" => Ok(Primitive::Polygon(12)),
            _ => Err("Could not parse".to_string()),
        }
    }
}
