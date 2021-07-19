use tagger::prelude::*;

fn main() -> core::fmt::Result {
    let width = 500.0;
    let height = 400.0;

    let mut root = tagger::Element::new(tagger::upgrade(std::io::stdout()));

    root.elem("svg", |header| {
        let svg = header.write(|b| {
            b.attr("xmlns", "http://www.w3.org/2000/svg")?
                .with_attr("viewBox", wr!("0 0 {} {}", width, height))
        })?;

        //Draw a path
        svg.single("polygon", |w| {
            w.attr("stroke", "black")?;
            w.attr("stroke-width", 2)?;
            w.attr("fill", "green")?;
            w.attr("fill-opacity", 0.5)?;

            w.points_data(|p| {
                p.add_point(100, 100)?;
                p.add_point(200, 100)?;
                p.add_point(300, 300)?;
                p.add_point(100, 200)
            })
        })
    })?;

    Ok(())
}
