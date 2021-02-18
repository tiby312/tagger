use tagger::prelude::*;

fn main() -> core::fmt::Result {
    let mut io = tagger::upgrade(std::io::stdout());

    tagger::single(&mut io, wr!("<!DOCTYPE html>"))?;
    let mut html = tagger::elem(&mut io, wr!("<html>"),wr!("</html>"))?;

    html.single(wr!("<style>{}</style>",
        "table, th, td {
      border: 1px solid black;
      border-collapse: collapse;
      animation: mymove 5s infinite;
    }
    @keyframes mymove {
        from {background-color: red;}
        to {background-color: blue;}
    }"
    ))?;

    let mut table = html.elem(wr!("<table style='width:{}%'>", 100),wr!("</table>"))?;

    for i in 0..20 {
        let mut tr = table.elem(wr!("<tr>"),wr!("</tr>"))?;

        tr.single(wr!("<th>Hay {}:1</th>", i))?;
        tr.single(wr!("<th>Hay {}:2</th>", i))?;
        tr.single(wr!("<th>Hay {}:3</th>", i))?;

        tr.end()?;
    }

    table.end()?;
    html.end()?;
    
    Ok(())
}
