use tagger::prelude::*;
fn main() -> std::fmt::Result {
    let width = 500.0;
    let height = 400.0;

    let w = &mut tagger::upgrade_write(std::io::stdout());

    element!(
        w,
        "svg",
        ("xmlns", "http://www.w3.org/2000/svg"),
        ("viewBox", format_args!("0 0 {} {}", width, height))
    )
    .build(|w| {
        single_element!(
            w,
            "polygon",
            ("stroke", "black"),
            ("stroke-width", 2),
            ("fill", "green"),
            ("fill-opacity", 0.5),
            (
                "points",
                points!((100, 100), (200, 100), (300, 300), (100, 200))
            )
        );
        Ok(())
    })
}
