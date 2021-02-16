use tagger::prelude::*;

fn main() {
    let mut string = String::new();

    {
        let mut root = tagger::root(&mut string);
        root.declaration("DOCTYPE html");

        let mut html = root.tag_build("html").end();

        //html.tag_build("rect").append("class='poloto2fill' height='7.5' rx='5' ry='5' width='50' x='680' y='176.25'").empty();

        html.tag_build("rect").set("width", 4).empty_no_slash();
        html.comment("test comment!");

        html.tag_build("rect")
            .set("class", "poloto2fill")
            .set("height", 7.5)
            .set("rx", 5)
            .empty();

        let mut style = html.tag_build("style").end();
        style.write_str(".potato{chicken}\n");
        drop(style);

        let mut div = html.tag_build("div").append("x=5").end();
        div.tag_build("svg").append("foo").end();
        div.tag_build("svg").append("blag").end();
        div.tag_build("img").append("width='100%'").empty();
        drop(div);

        html.tag_build("div").append("kiki=7").end();
    }
    use std::io::Write;
    std::io::stdout().write_all(&string.as_bytes()).unwrap();
}
