use tagger::*;
fn main() {
    let width = 100.0;
    let height = 100.0;

    let mut svg = {
        let svg_attr = new_attr()
            .attr("xmlns", "http://www.w3.org/2000/svg")
            .attr("viewBox", formatm!("0 0 {} {}", width, height))
            .finish();

        elem!("svg", svg_attr)
    };

    let rect = {
        let rect_attr = new_attr()
            .attr("x1", 0)
            .attr("y1", 0)
            .attr("rx", 20)
            .attr("ry", 20)
            .attr("width", width)
            .attr("height", height)
            .attr("style", "fill:blue")
            .finish();
        single!("rect", rect_attr)
    };

    svg.append(rect);

    let style = {
        let mut style = elem!("style");
        style.append(single!(".test{fill:none;stroke:white;stroke-width:3}"));
        style
    };

    svg.append(style);

    let g = {
        let gc = new_attr().attr("class", "test").finish();
        let mut g = elem!("g", gc);
        for r in (0..50).step_by(10) {
            let b = new_attr()
                .attr("cx", 50.0)
                .attr("cy", 50.0)
                .attr("r", r)
                .finish();

            g.append(single!("circle", b));
        }
        g
    };

    svg.append(g);

    println!("{}", svg);
}
