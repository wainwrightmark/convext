use std::{collections::BTreeMap, default};

use crate::core::prelude::*;
use itertools::Itertools;
use num::traits::ops::inv;
use pest::iterators::Pairs;
use pest::Parser;
use pest_derive::Parser;
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
    ) -> String {
        let rotate_transform = if relative_properties.r == 0.0 {
            "".to_string()
        } else {
            format!(
                "style=\"transform: rotate({r}deg);\"",
                r = relative_properties.r
            )
        };
        let color = format!(
            "fill=\"hsl({h}, {s}%, {l}%, {a}%)\" stroke=\"none\"",
            h = absolute_properties.h,
            s = absolute_properties.s * 100.0,
            l = absolute_properties.v * 100.0,
            a = absolute_properties.a * 100.0,
        );

        match self {
            Primitive::Circle => format!(
                "<ellipse cx={x} cy={y} rx={rx} ry={ry} {color} {rotate_transform} />",
                x = relative_properties.x,
                y = relative_properties.y,
                //ignore rotation
                rx = relative_properties.p * absolute_properties.w,
                ry = relative_properties.p * absolute_properties.l,
                color = color,
                rotate_transform = rotate_transform
            ),
            Primitive::Square => {
                let x = relative_properties.x - (relative_properties.p * absolute_properties.w);
                let y = relative_properties.y - (relative_properties.p * absolute_properties.l);

                let width = relative_properties.p * absolute_properties.w * 2.0;
                let height = relative_properties.p * absolute_properties.l * 2.0;

                let rx = relative_properties.p * absolute_properties.c;
                let ry = relative_properties.p * absolute_properties.c;

                format!("<rect x={x} y={y} width={width} height={height} rx={rx} ry={ry} {color}  {rotate_transform} />", 
                x=x,
                y=y,
                rx = rx,
                ry=ry,
                width=width,
                height=height,
                color= color,
                rotate_transform = rotate_transform
            )
            }
            Primitive::RightTriangle => {
                let points = [(0.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]
                    .into_iter()
                    .flat_map(|(x, y)| {
                        [
                            (x * relative_properties.p * absolute_properties.w)
                                + relative_properties.x,
                            (y * relative_properties.p * absolute_properties.l)
                                + relative_properties.y,
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
                            (x * relative_properties.p * absolute_properties.w)
                                + relative_properties.x,
                            (y * relative_properties.p * absolute_properties.l)
                                + relative_properties.y,
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
