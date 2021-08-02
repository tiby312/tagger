fn main() {
    let width = 100.0;
    let height = 100.0;

    let mut w = tagger::from_io(std::io::stdout());

    w.elem("svg", |d| {
        d.attr("xmlns", "http://www.w3.org/2000/svg")
            .attr("viewBox", format_args!("0 0 {} {}", width, height));
    })
    .build(|w| {
        w.single("rect", |d| {
            d.attr("x1", 0)
                .attr("y1", 0)
                .attr("rx", 20)
                .attr("ry", 20)
                .attr("width", width)
                .attr("height", height)
                .attr("style", "fill:blue");
        });

        w.elem("style", tagger::no_attr()).build(|w| {
            w.put_raw(".test{fill:none;stroke:white;stroke-width:3}");
        });

        w.elem("g", |d| {
            d.attr("class", "test");
        })
        .build(|w| {
            for r in (0..50).step_by(10) {
                w.single("circle", |w| {
                    w.attr("cx", 50.0).attr("cy", 50.0).attr("r", r);
                });
            }
        });
    });
}
