use tagger::element;

// This is a simple macro named `say_hello`.
macro_rules! one_thing {
    // `()` indicates that the macro takes no argument.
    ($a:expr) => {
        // The macro will expand into the contents of this block.
        element("","",$a);
    };
}

macro_rules! empty_elem {
    // `()` indicates that the macro takes no argument.
    ($a:tt) => {
        // The macro will expand into the contents of this block.
        element(concat!("<",$a,">"),concat!("</",$a,">"),"");
    };
}


fn main(){

    
    let mut svg=empty_elem!("svg");
    let mut g=empty_elem!("g");
    g.append(one_thing!("hello man"));
    g.append(one_thing!("hello man"));
    svg.append(g);
    svg.append(one_thing!("adfsadfs"));
    
    println!("{}",svg);
    
}
