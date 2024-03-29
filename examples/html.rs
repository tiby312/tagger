use tagger::no_attr;

fn main() -> std::fmt::Result {
    let mut w = tagger::new(tagger::upgrade_write(std::io::stdout()));

    w.put_raw_escapable("<!DOCTYPE html>")?;

    w.elem("style", no_attr())?.build(|w| {
        w.put_raw(
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

    w.elem("table", |d| d.attr("style", format_args!("width:{}%", 100)))?
        .build(|w| {
            for i in 0..20 {
                w.elem("tr", no_attr())?.build(|w| {
                    w.elem("th", no_attr())?
                        .build(|w| w.put_raw(format_args!("Hay {}:1", i)))?;
                    w.elem("th", no_attr())?
                        .build(|w| w.put_raw(format_args!("Hay {}:2", i)))?;
                    w.elem("th", no_attr())?
                        .build(|w| w.put_raw(format_args!("Hay {}:3", i)))
                })?;
            }
            Ok(())
        })
}
