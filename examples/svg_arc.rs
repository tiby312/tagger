fn main() -> std::fmt::Result {
    let width = 500.0;
    let height = 400.0;

    let mut w = tagger::new(tagger::upgrade_write(std::io::stdout()));

    w.elem("svg", |a| {
        a.attr("xmlns", "http://www.w3.org/2000/svg")?;
        a.attr("viewBox", format_args!("0 0 {} {}", width, height))
    })?
    .build(|w| {
        use tagger::PathCommand::*;
        w.single("path", |a| {
            a.attr("stroke", "black")?;
            a.attr("stroke-width", 2)?;
            a.attr("fill", "green")?;
            a.attr("fill-opacity", 0.5)?;
            a.path(|p| {
                p.put(M(200, 120))?;
                p.put(Q(300, 50, 400, 120))?;
                p.put(T(500, 120))
            })
        })?;

        w.single("path", |a| {
            a.attr("stroke", "black")?;
            a.attr("stroke-width", 2)?;
            a.attr("fill", "blue")?;
            a.attr("fill-opacity", 0.5)?;
            a.path(|p| {
                p.put(M(300, 200))?;
                p.put(H_(-150))?;
                p.put(A_(150, 150, 0, 1, 0, 150, -150))?;
                p.put(Z(""))
            })
        })
    })
}
