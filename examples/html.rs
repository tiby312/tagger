use tagger::prelude::*;
use tagger::tag_types;
fn main() -> core::fmt::Result {
    let mut root = tagger::Element::new(tagger::upgrade(std::io::stdout()));

    root.single_ext("DOCTYPE", tag_types::DECL, |a| {
        write_ret!(a, "html")?.empty_ok()
    })?;

    root.elem_no_attr("style", |style| {
        write!(
            style,
            "{}",
            "table, th, td {
            border: 1px solid black;
            border-collapse: collapse;
            animation: mymove 5s infinite;
          }
          @keyframes mymove {
              from {background-color: red;}
              to {background-color: blue;}
          }"
        )?;
        style.empty_ok()
    })?;

    root.elem("table", |header| {
        let (table, ()) =
            header.write(|w| w.with_attr("style", wr!("width:{}%", 100))?.empty_ok())?;

        for i in 0..20 {
            table.elem_no_attr("tr", |tr| {
                tr.elem_no_attr("th", |tr| {
                    write!(tr, "Hay {}:1", i)?;
                    tr.empty_ok()
                })?;
                tr.elem_no_attr("th", |tr| {
                    write!(tr, "Hay {}:2", i)?;
                    tr.empty_ok()
                })?;
                tr.elem_no_attr("th", |tr| {
                    write!(tr, "Hay {}:3", i)?;
                    tr.empty_ok()
                })
            })?;
        }
        table.empty_ok()
    })?;

    Ok(())
}
