use tagger::prelude::*;

fn main() -> std::fmt::Result {
    use std::fmt::Write;

    let w = &mut tagger::upgrade_write(std::io::stdout());

    let mut w = tagger::ElemWriter::new(w);

    write!(w.writer(), "{}", "<!DOCTYPE html>")?;

    element!(w, "style")?.build(|w| {
        write!(
            w.writer(),
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

    element!(w, "table", ("style", format_args!("width:{}%", 100)))?.build(|w| {
        for i in 0..20 {
            element!(w, "tr")?.build(|w| {
                element!(w, "th")?.build(|w| write!(w.writer(), "Hay {}:1", i))?;
                element!(w, "th")?.build(|w| write!(w.writer(), "Hay {}:2", i))?;
                element!(w, "th")?.build(|w| write!(w.writer(), "Hay {}:3", i))
            })?;
        }
        Ok(())
    })
}
