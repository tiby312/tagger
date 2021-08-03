fn main() {
    let width = 500.0;
    let height = 400.0;

    let mut w = tagger::new(tagger::upgrade_write(std::io::stdout()));

    w.elem("svg", |a| {
        a.attr("xmlns", "http://www.w3.org/2000/svg")
            .attr("viewBox", format_args!("0 0 {} {}", width, height));
    })
    .build(|w| {
        use tagger::PathCommand::*;
        w.single("path", |a| {
            a.attr("stroke", "black")
                .attr("stroke-width", 2)
                .attr("fill", "green")
                .attr("fill-opacity", 0.5)
                .path(|p| {
                    p.put(M(200, 120))
                        .put(Q(300, 50, 400, 120))
                        .put(T(500, 120));
                });
        });

        w.single("path", |a| {
            a.attr("stroke", "black")
                .attr("stroke-width", 2)
                .attr("fill", "blue")
                .attr("fill-opacity", 0.5)
                .path(|p| {
                    p.put(M(300, 200))
                        .put(H_(-150))
                        .put(A_(150, 150, 0, 1, 0, 150, -150))
                        .put(Z(""));
                });
        });
    });
}
