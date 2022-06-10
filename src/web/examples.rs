pub struct Example{
    pub name: &'static str,
    pub text: &'static str
}

pub const EXAMPLES: [Example; 4] = 

[
Example{
    name: "New",
    text:
    "circle v0.5",
},

Example{
    
    
    name: "Circles", text:
"myshape
rul myshape
circle v 0.5
myshape p 0.75 h 40
end"},
Example{name: "Face", text:
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
circle w 0.5 l 0.9 p 0.5 v 0.8 h 235"
},

Example{name: "Pascal", text:"let hue 40
pascal
rul pascal
triangle v0.5
pascal h ?hue p 0.5 ym0.5
pascal h ?hue p 0.5 y0.25 x0.5
pascal h ?hue p 0.5 y0.25 xm0.5"},

];