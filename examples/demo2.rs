use tagger::prelude::*;

fn main() {
    let mut s = String::new();

    {
        let root = tagger::root(&mut s);

        let svg = root.tag_build_flat("svg").set("width", 100).end();
        let svg = svg.tag_build_flat("img").set("height", 100).empty();
        let div = svg.tag_build_flat("div").end();
        let svg = div.pop();
        let mut svg = svg.tag_build_flat("chicken").empty();
        svg.tag_build("potato").set("foo", 5).empty();
    }

    println!("{}", s);
}
