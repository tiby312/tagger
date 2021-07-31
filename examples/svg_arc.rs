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
        use tagger::PathCommand::*;

        single_element!(
            w,
            "path",
            ("stroke", "black"),
            ("stroke-width", 2),
            ("fill", "green"),
            ("fill-opacity", 0.5),
            ("d", path!(M(200, 120), Q(300, 50, 400, 120), T(500, 120)))
        );

        single_element!(
            w,
            "path",
            ("stroke", "black"),
            ("stroke-width", 2),
            ("fill", "blue"),
            ("fill-opacity", 0.5),
            (
                "d",
                path!(
                    M(300, 200),
                    H_(-150),
                    A_(150, 150, 0, 1, 0, 150, -150),
                    Z("")
                )
            )
        );

        Ok(())
    })
}
