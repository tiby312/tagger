use std::fmt;
use tagger::prelude::*;

fn make_image() -> impl fmt::Display {
    let mut svg = elem!("svg");

    let k = "my string!".to_string();
    svg.append(single!(k));

    svg.appendm(elem!("g1").appendm(elem!("g2")))
}

fn main() {
    println!("{}", make_image());
}
