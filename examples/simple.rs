use tagger::*;
fn main() {
    let width = 100.0;
    let height = 100.0;

    let mut svg = {
        let svg_attr = AttrBuilder::new()
            .attr("xmlns", "http://www.w3.org/2000/svg")
            .attr("viewBox", move_format!("0 0 {} {}", width, height))
            .finish();

        element(move_format!("<svg {}>", svg_attr), "</svg>")
    };

    let rect = {
        let rect_attr = AttrBuilder::new()
            .attr("x1", 0)
            .attr("y1", 0)
            .attr("rx", 20)
            .attr("ry", 20)
            .attr("width", width)
            .attr("height", height)
            .attr("style", "fill:blue")
            .finish();
        elem_single!(move_format!("<rect {}/>", rect_attr))
    };

    svg.append(rect);

    let style={
        let mut style = element("<style>", "</style>");
        style.append(elem_single!(".test{fill:none;stroke:white;stroke-width:3}"));
        style
    };

    svg.append(style);

    let g={
        let gc = AttrBuilder::new().attr("class", "test").finish();
        let mut g = element(move_format!("<g {}>", gc), "</g>");
        for r in (0..50).step_by(10) {
            let b = AttrBuilder::new()
                .attr("cx", 50.0)
                .attr("cy", 50.0)
                .attr("r", r)
                .finish();

            g.append(elem_single!(move_format!("<circle {}/>", b)));
        }
        g
    };

    svg.append(g);

    println!("{}", svg);
}
