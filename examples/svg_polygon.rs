fn main() -> std::fmt::Result {
    let width = 500.0;
    let height = 400.0;

    let mut w = tagger::new(tagger::upgrade_write(std::io::stdout()));

    w.elem("svg", |d| {
        d.attr("xmlns", "http://www.w3.org/2000/svg")?;
        d.attr("viewBox", format_args!("0 0 {} {}", width, height))
    })?
    .build(|w| {
        w.single("polygon", |d| {
            d.attr("stroke", "black")?;
            d.attr("stroke-width", 2)?;
            d.attr("fill", "green")?;
            d.attr("fill-opacity", 0.5)?;
            d.points(|p| {
                p.put(100, 100)?;
                p.put(200, 100)?;
                p.put(300, 300)?;
                p.put(100, 200)
            })
        })
    })
}
