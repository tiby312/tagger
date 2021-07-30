use tagger::prelude::*;

fn main() {
    let width = 100.0;
    let height = 100.0;

    let mut svg = elem!(
        "svg",
        ("xmlns", "http://www.w3.org/2000/svg"),
        ("viewBox", formatm!("0 0 {} {}", width, height))
    );

    svg.append(single!(
        "rect",
        ("x1", 0),
        ("y1", 0),
        ("rx", 20),
        ("ry", 20),
        ("width", width),
        ("height", height),
        ("style", "fill:blue")
    ));

    svg.append(elem!("style").appendm(".test{fill:none;stroke:white;stroke-width:3}"));

    let mut g = elem!("g", ("class", "test"));
    for r in (0..50).step_by(10) {
        g.append(single!("circle", ("cx", 50.0), ("cy", 50.0), ("r", r)));
    }
    svg.append(g);

    println!("{}", svg.display());
}
