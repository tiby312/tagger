use tagger::prelude::*;

use tagger::*;
fn main() {
    let width = 100.0;
    let height = 100.0;

    let svg_attr = AttrBuilder::new()
        .attr("xmlns", "http://www.w3.org/2000/svg")
        .attr("viewBox", move_format!("0 0 {} {}", width, height))
        .finish();

    let mut svg = element(move_format!("<svg {}>", svg_attr), "</svg>");

    let rect_attr = AttrBuilder::new()
        .attr("x1", 0)
        .attr("y1", 0)
        .attr("rx", 20)
        .attr("ry", 20)
        .attr("width", width)
        .attr("height", height)
        .attr("style", "fill:blue")
        .finish();

    svg.append_single(move_format!("<rect {}/>", rect_attr));

    let mut e = element("<style>", "</style>");
    e.append(elem_single!(".test{fill:none;stroke:white;stroke-width:3}"));
    svg.append(e);

    let gc = AttrBuilder::new().attr("class", "test").finish();
    let mut g = element(move_format!("<g {}>", gc), "</g>");
    for r in (0..50).step_by(10) {
        let b = AttrBuilder::new()
            .attr("cx", 50.0)
            .attr("cy", 50.0)
            .attr("r", r)
            .finish();

        g.append_single(move_format!("<circle {}/>", b));
    }

    svg.append(g);

    println!("{}", svg);
}

fn main() -> core::fmt::Result {
    let width = 500.0;
    let height = 400.0;

    let mut root = tagger::Element::new(tagger::upgrade(std::io::stdout()));

    root.elem("svg", |header| {
        let (svg, ()) = header.write(|b| {
            b.attr("xmlns", "http://www.w3.org/2000/svg")?
                .with_attr("viewBox", wr!("0 0 {} {}", width, height))?
                .empty_ok()
        })?;

        //Draw a path
        svg.single("polygon", |w| {
            w.attr("stroke", "black")?;
            w.attr("stroke-width", 2)?;
            w.attr("fill", "green")?;
            w.attr("fill-opacity", 0.5)?;

            w.points_data(|p| {
                p.add_point(100, 100)?;
                p.add_point(200, 100)?;
                p.add_point(300, 300)?;
                p.add_point(100, 200)
            })?;

            w.empty_ok()
        })?;

        svg.empty_ok()
    })?;

    Ok(())
}
