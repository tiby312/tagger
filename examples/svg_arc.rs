
fn main() {
    let width = 500.0;
    let height = 400.0;

    let mut w= tagger::from_io(std::io::stdout());

    w.elem("svg", |a| {
        a.attr("xmlns", "http://www.w3.org/2000/svg");
        a.attr("viewBox", format_args!("0 0 {} {}", width, height))
    })
    .build(|w| {
        use tagger::PathCommand::*;
        w.single("path", |a| {
            a.attr("stroke", "black");
            a.attr("stroke-width", 2);
            a.attr("fill", "green");
            a.attr("fill-opacity", 0.5);

            a.path(|p|{
                p.add(M(200, 120));
                p.add(Q(300, 50, 400, 120));
                p.add(T(500, 120))
            })
        });

        w.single("path", |a| {
            a.attr("stroke", "black");
            a.attr("stroke-width", 2);
            a.attr("fill", "blue");
            a.attr("fill-opacity", 0.5);

            a.path(|p|{
                p.add(M(300, 200));
                p.add(H_(-150));
                p.add(A_(150, 150, 0, 1, 0, 150, -150));
                p.add(Z(""))
            })
        })
    })
}
