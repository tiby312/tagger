fn main() {
    let width = 500.0;
    let height = 400.0;

    let mut w = tagger::from_io(std::io::stdout());

    w.elem("svg", |d| {
        d.attr("xmlns", "http://www.w3.org/2000/svg")
            .attr("viewBox", format_args!("0 0 {} {}", width, height));
    })
    .build(|w| {
        w.single("polygon", |d| {
            d.attr("stroke", "black")
                .attr("stroke-width", 2)
                .attr("fill", "green")
                .attr("fill-opacity", 0.5)
                .points(|p| {
                    p.add(100, 100).add(200, 100).add(300, 300).add(100, 200);
                });
        });
    });
}
