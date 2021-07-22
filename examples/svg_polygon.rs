use tagger::prelude::*;
use tagger::{attr_builder, points_builder};
fn main() {
    let width = 500.0;
    let height = 400.0;

    let mut svg = {
        let svg_attr = attr_builder()
            .attr("xmlns", "http://www.w3.org/2000/svg")
            .attr("viewBox", formatm!("0 0 {} {}", width, height))
            .build();

        elem!("svg", svg_attr)
    };

    let polygon = {
        let polygon = points_builder()
            .add(100, 100)
            .add(200, 100)
            .add(300, 300)
            .add(100, 200)
            .build();

        let gc = attr_builder()
            .attr("stroke", "black")
            .attr("stroke-width", 2)
            .attr("fill", "green")
            .attr("fill-opacity", 0.5)
            .attr_whole(polygon)
            .build();

        elem!("polygon", gc)
    };

    svg.append(polygon);

    println!("{}", svg);
}
