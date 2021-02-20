use tagger::prelude::*;

fn main() -> core::fmt::Result {
    let mut io = tagger::upgrade(std::io::stdout());

    let mut root = tagger::xml::xml(&mut io)?;

    root.declaration("DOCTYPE", wr!("html"))?;

    let mut html = root.elem_no_attr("html")?;

    html.elem_no_attr("style")?.defer_end(|style| {
        style.inner_str(
            "table, th, td {
          border: 1px solid black;
          border-collapse: collapse;
          animation: mymove 5s infinite;
        }
        @keyframes mymove {
            from {background-color: red;}
            to {background-color: blue;}
        }",
        )
    })?;

    let mut table = html.elem("table", |w| w.with_attr("style", wr!("width:{}%", 100)))?;

    for i in 0..20 {
        let mut tr = table.elem_no_attr("tr")?;

        tr.elem_no_attr("th")?
            .move_inner(wr!("Hay {}:1", i))?
            .end()?;
        tr.elem_no_attr("th")?
            .move_inner(wr!("Hay {}:2", i))?
            .end()?;
        tr.elem_no_attr("th")?
            .move_inner(wr!("Hay {}:3", i))?
            .end()?;

        tr.end()?;
    }

    table.end()?;
    html.end()?;
    root.end()?;
    Ok(())
}
