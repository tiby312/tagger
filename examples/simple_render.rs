use tagger::render::{self, *};
fn main() -> std::fmt::Result {
    let width = 100.0;
    let height = 100.0;

    let rect=elem!("g",("x1",0),("y1",0),("rx",20),("ry",20),("width",width),("height",height),("style","fill:blue"));

    let style=elem!("style").put_raw(".test{fill:none;stroke:white;stroke-width:3}");

    let svg=elem!("svg",("xmlns", "http://www.w3.org/2000/svg"),("viewBox", format_args!("0 0 {} {}", width, height)));

    let table=elem!("g",("class","test")).append(dyn_elem(|w|{
        for r in (0..50).step_by(10) {
            w.elem_render(render::single("circle", |w| {
                w.attr("cx", 50.0)?;
                w.attr("cy", 50.0)?;
                w.attr("r", r)
            }))?;
        }
        Ok(())
    }));

    let all = svg.append(style).append(rect).append(table);

    let mut w = tagger::upgrade_write(std::io::stdout());
    all.render_all(&mut w)
}
