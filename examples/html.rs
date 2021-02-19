use tagger::prelude::*;

fn main() -> core::fmt::Result {
    let mut io = tagger::upgrade(std::io::stdout());

    let mut root=tagger::xml(&mut io)?;

    root.declaration("DOCTYPE",wr!("html"))?;

    let mut html=root.elem_simple("html")?;

    let mut style=html.elem_simple("style")?;
    style.inner_str(
        "table, th, td {
      border: 1px solid black;
      border-collapse: collapse;
      animation: mymove 5s infinite;
    }
    @keyframes mymove {
        from {background-color: red;}
        to {background-color: blue;}
    }")?;
    style.end()?;


    let mut table = html.elem("table",|w|
    w.with_attr("style",wr!("width:{}%", 100)))?;

    for i in 0..20 {
        let mut tr = table.elem_simple("tr")?;

        tr.elem_simple("th")?.move_inner(wr!("Hay {}:1",i))?.end()?;
        tr.elem_simple("th")?.move_inner(wr!("Hay {}:2",i))?.end()?;
        tr.elem_simple("th")?.move_inner(wr!("Hay {}:3",i))?.end()?;

        tr.end()?;
    }

    table.end()?;
    html.end()?;
    root.end()?;
    Ok(())
}
