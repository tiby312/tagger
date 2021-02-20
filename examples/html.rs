use tagger::prelude::*;
use tagger::xml::tag_types;
fn main() -> core::fmt::Result {
    let mut root = tagger::xml::Element::new(tagger::upgrade(std::io::stdout()));

    root.single_ext("DOCTYPE", tag_types::DECL, |a| {
        write!(a, "html")?;
        a.ok()
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

    root.elem("table", |builder| {
        let table = builder.build(|w| w.with_attr("style", wr!("width:{}%", 100)))?;

        for i in 0..20 {
            table.elem("tr", |builder| {
                let tr = builder.build(|e| e.ok())?;

                tr.elem_no_attr("th", |tr| write!(tr, "Hay {}:1", i))?;
                tr.elem_no_attr("th", |tr| write!(tr, "Hay {}:2", i))?;
                tr.elem_no_attr("th", |tr| write!(tr, "Hay {}:3", i))?;

                tr.ok()
            })?;
        }

        table.ok()
    })
}
