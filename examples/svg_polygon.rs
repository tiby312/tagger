use tagger::*;
fn main() {
    let width = 500.0;
    let height = 400.0;

    let mut svg = {
        let svg_attr = AttrBuilder::new()
            .attr("xmlns", "http://www.w3.org/2000/svg")
            .attr("viewBox", move_format!("0 0 {} {}", width, height))
            .finish();

        element(move_format!("<svg {}>", svg_attr), "</svg>")
    };

    let polygon = {
        let polygon = PointsBuilder::new()
            .add(100, 100)
            .add(200, 100)
            .add(300, 300)
            .add(100, 200)
            .finish();

        let gc = AttrBuilder::new()
            .attr("stroke", "black")
            .attr("stroke-width", 2)
            .attr("fill", "green")
            .attr("fill-opacity", 0.5)
            .attr_raw(polygon)
            .finish();

        elem_single!(move_format!("<polygon {}/>", gc))
    };

    svg.append(polygon);

    println!("{}", svg);
}
