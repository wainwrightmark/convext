use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Creation {
    pub name: String,
    pub text: String,
}

pub const EXAMPLES: [(&str, &str); 6] = [
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
        ear xsub0.7
        pentagon r180 v0.8  w0.9
        eye p 0.2 xsub0.3ysub0.3
        eye p 0.2 x0.3ysub0.3
        lips y0.6
        nose
        
        cheek x0.55
        cheek xsub0.55
        hair
        hair x0.1
        hair x0.2
        hair x0.3
        hair xsub0.1
        hair xsub0.2
        hair xsub0.3
        
        rul hair
        triangle p0.1 w2.8 ysub0.85
        
        rul ear
        hexagon p0.2 v0.8 w0.5  ysub0.25
        
        rul cheek
        circle p0.2 v0.4 a0.5 w0.5 y0.25
        
        rul nose
        rtriangle p0.3 w0.5 v0.7 h 350
        
        rul lips
        octagon p0.2 w2.0 v0.4
        square p0.05 w4.0 v0.2
        
        rul eye
        square ysub1 l0.25
        circle w 0.5 l 0.9 v 0.9 r  90
        circle w 0.5 l 0.9 p 0.5 v 0.8 h 235",
    ),
    (
        "Pascal",
        "let hue 40
        pascal
        rul pascal
        triangle v0.5
        pascal h ?hue p 0.5 ysub0.5
        pascal h ?hue p 0.5 y0.25 x0.5
        pascal h ?hue p 0.5 y0.25 xsub0.5",
    ),
    (
        "Tree",
        "grow y0.9 p0.02 v0.5 h120 a 0.5

let probBranch 0.1
let angleRight 15
let angleLeft 330

rul grow ?probBranch
grow  r ?angl
grow  r ?angr

rul grow
square l 4 w0.5 ysub5
grow ysub8 p0.99 a0.9
",
    ),
    (
        "Maze",
        "maze  r45

        rul maze 
        row p 0.25 x-1 y-1
        rul row
        column
        row y 1
        
        rul column
        shape p0.5
        column x 1
        
        rul shape 0.5
        square w 0.1 l1.5 r 45
        
        rul shape
        square w 0.1 l1.5 r sub45"

    )
];
