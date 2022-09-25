use tagger::elem;
use tagger::render::*;
use tagger::single;

fn main() -> std::fmt::Result {
    let width = 100.0;
    let height = 100.0;

    let rect = single!(
        "rect",
        ("x1", 0),
        ("y1", 0),
        ("rx", 20),
        ("ry", 20),
        ("width", width),
        ("height", height),
        ("style", "fill:blue")
    );

    let style = elem!("style").put_raw(".test{fill:none;stroke:white;stroke-width:3}");

    let svg = elem!(
        "svg",
        ("xmlns", "http://www.w3.org/2000/svg"),
        ("viewBox", format!("0 0 {} {}", width, height))
    );

    let table = elem!("g", ("class", "test")).append(dyn_elem(|w| {
        for r in (0..50).step_by(10) {
            let circle = single!("circle", ("cx", 50.0), ("cy", 50.0), ("r", r));
            w.elem_render(circle)?;
        }
        Ok(())
    }));

    let all = svg.append(style).append(rect).append(table);

    let w = tagger::upgrade_write(std::io::stdout());
    all.render_with(w)
}
