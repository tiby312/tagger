use tagger::attrs;
use tagger::render::*;

fn main() -> std::fmt::Result {
    let width = 100.0;
    let height = 100.0;

    let rect = single("rect").attr(attrs!(
        ("x1", 0),
        ("y1", 0),
        ("rx", 20),
        ("ry", 20),
        ("width", width),
        ("height", height),
        ("style", "fill:blue")
    ));

    let style = elem("style").append(raw(".test{fill:none;stroke:white;stroke-width:3}"));

    let svg = elem("svg").attr(attrs!(
        ("xmlns", "http://www.w3.org/2000/svg"),
        ("viewBox", format!("0 0 {} {}", width, height))
    ));

    let rows = (0..50)
        .step_by(10)
        .map(|r| single("circle").attr(attrs!(("cx", 50.0), ("cy", 50.0), ("r", r))));

    let table = elem("g").attr(("class", "test")).append(rows);

    let all = svg.append(style).append(rect).append(table);

    let w = tagger::upgrade_write(std::io::stdout());
    all.render_with(w)
}
