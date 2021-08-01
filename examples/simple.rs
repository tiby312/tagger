use std::fmt::Write;
use tagger::prelude::*;

fn main() -> std::fmt::Result {
    let width = 100.0;
    let height = 100.0;
    let w = &mut tagger::upgrade_write(std::io::stdout());

    let mut w = tagger::ElemWriter::new(w);

    element!(
        w,
        "svg",
        ("xmlns", "http://www.w3.org/2000/svg"),
        ("viewBox", format_args!("0 0 {} {}", width, height))
    )?
    .build(|w| {
        single_element!(
            w,
            "rect",
            ("x1", 0),
            ("y1", 0),
            ("rx", 20),
            ("ry", 20),
            ("width", width),
            ("height", height),
            ("style", "fill:blue")
        )?;

        element!(w, "style")?.build(|w| {
            write!(
                w.writer(),
                "{}",
                ".test{fill:none;stroke:white;stroke-width:3}"
            )
        })?;

        element!(w, "g", ("class", "test"))?.build(|w| {
            for r in (0..50).step_by(10) {
                single_element!(w, "circle", ("cx", 50.0), ("cy", 50.0), ("r", r))?;
            }
            Ok(())
        })
    })
}
