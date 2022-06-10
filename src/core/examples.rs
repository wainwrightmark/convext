use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Creation {
    pub name: String,
    pub text: String,
}

pub const EXAMPLES: [(&str, &str); 5] = [
    ("New", "circle v0.5"),
    (
        "Circles",
        "myshape
rul myshape
circle v 0.5
myshape p 0.75 h 40
end",
    ),
    (
        "Face",
        "ear x0.7
ear xm0.7
pentagon r180 v0.8  w0.9
eye p 0.2 xm0.3ym0.3
eye p 0.2 x0.3ym0.3
lips y0.6
nose

cheek x0.55
cheek xm0.55
hair
hair x0.1
hair x0.2
hair x0.3
hair xm0.1
hair xm0.2
hair xm0.3

rul hair
triangle p0.1 w2.8 ym0.85

rul ear
hexagon p0.2 v0.8 w0.5  ym0.25

rul cheek
circle p0.2 v0.4 a0.5 w0.5 y0.25

rul nose
rtriangle p0.3 w0.5 v0.7 h 350

rul lips
octagon p0.2 w2.0 v0.4
square p0.05 w4.0 v0.2

rul eye
square ym1 l0.25
circle w 0.5 l 0.9 v 0.9 r  90
circle w 0.5 l 0.9 p 0.5 v 0.8 h 235",
    ),
    (
        "Pascal",
        "let hue 40
pascal
rul pascal
triangle v0.5
pascal h ?hue p 0.5 ym0.5
pascal h ?hue p 0.5 y0.25 x0.5
pascal h ?hue p 0.5 y0.25 xm0.5",
    ),
    ("Tree",

"line y0.9 p0.01 v0.5 h120 a 0.5

let probr 0.3
let probl 0.3
let angr 15
let angl 330

rul line
square l 4 w0.5 ym5
line ym8 p0.99 a0.9

rul line ?probl
line ym4 r ?angl

rul line ?probr
line ym4 r ?angr
"
)
];
