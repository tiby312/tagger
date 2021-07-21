use tagger::*;
fn main() {
    let width = 500.0;
    let height = 400.0;

    let mut svg = {
        let svg_attr = attr_list()
            .attr("xmlns", "http://www.w3.org/2000/svg")
            .attr("viewBox", formatm!("0 0 {} {}", width, height))
            .build();

        elem!("svg", svg_attr)
    };

    let polygon = {
        let polygon = new_points()
            .add(100, 100)
            .add(200, 100)
            .add(300, 300)
            .add(100, 200)
            .build();

        let gc = attr_list()
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
