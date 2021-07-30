use tagger::prelude::*;
fn main() {
    let width = 500.0;
    let height = 400.0;

    let mut svg = elem!(
        "svg",
        ("xmlns", "http://www.w3.org/2000/svg"),
        ("viewBox", formatm!("0 0 {} {}", width, height))
    );

    svg.append(elem!(
        "polygon",
        ("stroke", "black"),
        ("stroke-width", 2),
        ("fill", "green"),
        ("fill-opacity", 0.5),
        (
            "points",
            points!((100, 100), (200, 100), (300, 300), (100, 200))
        )
    ));

    println!("{}", svg.display());
}
