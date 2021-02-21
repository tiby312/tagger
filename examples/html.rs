use tagger::prelude::*;
use tagger::tag_types;
fn main() -> core::fmt::Result {
    let mut root = tagger::Element::new(tagger::upgrade(std::io::stdout()));

    root.single_ext("DOCTYPE", tag_types::DECL, |a| {
        write!(a, "html")?;
        Ok(a)
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
        )
    })?;

    root.elem("table", |header| {
        let (table, cert) = header.write(|w| w.with_attr("style", wr!("width:{}%", 100)))?;

        for i in 0..20 {
            table.elem("tr", |header| {
                let (tr, cert) = header.write(|e| Ok(e))?;

                tr.elem_no_attr("th", |tr| write!(tr, "Hay {}:1", i))?;
                tr.elem_no_attr("th", |tr| write!(tr, "Hay {}:2", i))?;
                tr.elem_no_attr("th", |tr| write!(tr, "Hay {}:3", i))?;

                cert
            })?;
        }

        cert
    })
}
