
use tagger::prelude::*;
use std::fmt;

fn make_image()->impl fmt::Display{
    let mut svg=elem!("svg");
    
    let k="my string!".to_string();
    svg.append(single!(k));
   
    svg.add(elem!("g1").add(elem!("g2")))
}


fn main(){
    println!("{}",make_image());
}