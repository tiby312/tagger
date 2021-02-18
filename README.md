### Overview

Build xml / html / svg programatically using element building blocks.

Find it on [github](https://github.com/tiby312/tagger) and [crates.io](https://crates.io/crates/tagger).

### Example

```rust
use tagger::prelude::*;

fn main() -> core::fmt::Result {
    let mut io = tagger::upgrade(std::io::stdout());
    let width = 100.0;
    let height = 100.0;

    let mut svg = tagger::elem(
        &mut io,
        wr!(
            "<svg viewBox='0 0 {} {}' xmlns='http://www.w3.org/2000/svg'>",
            width,
            height
        ),
        wr!("</svg>"),
    )?;

    svg.single(wr!(
        "{}",
        "<style>.test{fill:none;stroke:white;stroke-width:3}</style>"
    ))?;

    svg.single(wr!(
        "<rect width='{}' height='{}' rx='{}' yx='{}' style='fill:blue;'/>",
        width,
        height,
        20,
        20
    ))?;

    let mut g = svg.elem(wr!("<g class='test'>"), wr!("</g>"))?;
    for r in (0..50).step_by(10) {
        g.single(wr!("<circle cx='{}' cy='{}' r='{}'/>", 50.0, 50.0, r))?;
    }

    //Program panics if the elements aren't closed.
    g.end()?;
    svg.end()?;
    Ok(())
}

```




### Output


<img src="./assets/svg_example.svg" alt="demo">
