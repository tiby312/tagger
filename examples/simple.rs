fn main() -> std::fmt::Result {
    let width = 100.0;
    let height = 100.0;

    let mut w = tagger::new(tagger::upgrade_write(std::io::stdout()));

    w.elem("svg", |d| {
        d.attr("xmlns", "http://www.w3.org/2000/svg")?;
        d.attr("viewBox", format_args!("0 0 {} {}", width, height))
    })?
    .build(|w| {
        w.single("rect", |d| {
            d.attr("x1", 0)?;
            d.attr("y1", 0)?;
            d.attr("rx", 20)?;
            d.attr("ry", 20)?;
            d.attr("width", width)?;
            d.attr("height", height)?;
            d.attr("style", "fill:blue")
        })?;

        w.elem("style", tagger::empty_attr)?
            .build(|w| w.put_raw(".test{fill:none;stroke:white;stroke-width:3}"))?;

        w.elem("g", |d| d.attr("class", "test"))?.build(|w| {
            for r in (0..50).step_by(10) {
                w.single("circle", |w| {
                    w.attr("cx", 50.0)?;
                    w.attr("cy", 50.0)?;
                    w.attr("r", r)
                })?;
            }
            Ok(())
        })
    })
}
