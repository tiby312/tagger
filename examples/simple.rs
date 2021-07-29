use tagger::attr_builder;
use tagger::prelude::*;

fn main() {
    let width = 100.0;
    let height = 100.0;

    let mut svg = elem!(
        "svg",
        attr_builder()
            .attr("xmlns", "http://www.w3.org/2000/svg")
            .attr("viewBox", formatm!("0 0 {} {}", width, height))
            .build()
    );

    let rect_attr = attr_builder()
        .attr("x1", 0)
        .attr("y1", 0)
        .attr("rx", 20)
        .attr("ry", 20)
        .attr("width", width)
        .attr("height", height)
        .attr("style", "fill:blue")
        .build();

    svg.append(single!("rect", rect_attr));

    svg.append(elem!("style").appendm(".test{fill:none;stroke:white;stroke-width:3}"));

    let mut g = elem!("g", attr_builder().attr("class", "test").build());
    for r in (0..50).step_by(10) {
        g.append(single!(
            "circle",
            attr_builder()
                .attr("cx", 50.0)
                .attr("cy", 50.0)
                .attr("r", r)
                .build()
        ));
    }
    svg.append(g);


    println!("{}", svg.display());
}
