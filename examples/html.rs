use tagger::prelude::*;
use tagger::tag_types;
fn main() -> core::fmt::Result {
    let mut root = tagger::Element::new(tagger::upgrade(std::io::stdout()));

    root.single_ext("DOCTYPE", tag_types::DECL, |a| {
        write_ret!(a, "html")
    })?;

    root.elem_no_attr("style", |style| {
        write_ret!(
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
        )
    })?;

    root.elem("table", |header| {
        let table = header.write(|w| w.with_attr("style", wr!("width:{}%", 100)))?;

        for i in 0..20 {
            table.elem("tr", |header| {
                let tr = header.write(|e| Ok(e))?;

                tr.elem_no_attr("th", |tr| {write!(tr, "Hay {}:1", i)?;Ok(tr)})?;
                tr.elem_no_attr("th", |tr| {write!(tr, "Hay {}:2", i)?;Ok(tr)})?;
                tr.elem_no_attr("th", |tr| {write!(tr, "Hay {}:3", i)?;Ok(tr)})?;

                Ok(tr)
            })?;
        }

        Ok(table)
    })?;

    Ok(())
}
