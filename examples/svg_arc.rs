use tagger::prelude::*;

fn main() {
    let width = 500.0;
    let height = 400.0;

    let mut svg = elem!(
        "svg",
        ("xmlns", "http://www.w3.org/2000/svg"),
        ("viewBox", formatm!("0 0 {} {}", width, height))
    );

    use tagger::PathCommand::*;

    svg.append(elem!(
        "path",
        ("stroke", "black"),
        ("stroke-width", 2),
        ("fill", "green"),
        ("fill-opacity", 0.5),
        ("d", path!(M(200, 120), Q(300, 50, 400, 120), T(500, 120)))
    ));

    svg.append(elem!(
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
    ));

    println!("{}", svg.display());
}
