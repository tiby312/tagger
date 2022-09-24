use tagger::render::{self, RenderElem};
fn main() -> std::fmt::Result {
    let width = 100.0;
    let height = 100.0;

    let rect = render::single("rect", |d| {
        d.attr("x1", 0)?;
        d.attr("y1", 0)?;
        d.attr("rx", 20)?;
        d.attr("ry", 20)?;
        d.attr("width", width)?;
        d.attr("height", height)?;
        d.attr("style", "fill:blue")
    });

    let style = render::elem("style", tagger::empty_attr)
        .put_raw(".test{fill:none;stroke:white;stroke-width:3}");

    let svg = render::elem("svg", |d| {
        d.attr("xmlns", "http://www.w3.org/2000/svg")?;
        d.attr("viewBox", format_args!("0 0 {} {}", width, height))
    });

    let table = render::elem("g", |d| d.attr("class", "test")).add(|w| {
        for r in (0..50).step_by(10) {
            w.single("circle", |w| {
                w.attr("cx", 50.0)?;
                w.attr("cy", 50.0)?;
                w.attr("r", r)
            })?;
        }
        Ok(())
    });

    let all = svg.append(style).append(rect).append(table);

    let mut w = tagger::upgrade_write(std::io::stdout());
    all.render_all(&mut w)
}
