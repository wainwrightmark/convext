use convext::core::prelude::*;

use ntest::test_case;
// use rand::{prelude::StdRng, Rng};

pub const EXAMPLES: [&str; 5] = [
    "Circle",
    "let myvar 100
square h ?myvar",
    "circle circle p 0.5 h 120",
    "myshape
rul myshape
circle
myshape p 0.75 h 40
end",
    "
blackshape
rul blackshape
square h 120
whiteshape p 0.5 x m0.5 y m0.5
whiteshape p 0.5 x 0.5 y 0.5
end

rul whiteshape
square
blackshape p 0.5 x m0.5 y m0.5
blackshape p 0.5 x 0.5 y 0.5
end",
];

#[test_case(0)]
#[test_case(1)]
#[test_case(2)]
#[test_case(3)]
#[test_case(4)]
fn test_svg(index: usize) {
    let input = EXAMPLES[index];
    let grammar = parse(input).unwrap();

    let node = grammar.to_root_node(ExpandSettings::default());

    let svg = node.to_svg(&grammar);

    assert!(!svg.is_empty());
    //print!("\r\n{svg}\r\n");
}
