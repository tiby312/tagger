### Overview

Build xml / html / svg programatically using element building blocks.
Instead of using a templating engine, write data/markup that 'looks like' rust.

Find it on [github](https://github.com/tiby312/tagger) and [crates.io](https://crates.io/crates/tagger).

### Features

Tagger aims to be memory efficient. Instead of constructing a nested structure of tags in memory and then have it be written out,
Tagger will write out the elements to a `fmt::Write` object on the fly. 

Tagger aims to guarantee correct writing of elements at compile time. At compile time, Tagger ensures that
every tag has zero or more attributes followed by symbol to complete that tag, and that every tag that needs an ending tag, has one. This is achieved
by nesting closures. That said this isn't 100% true. The user is allowed to write arbitrary data inside of any element,
so it is possible that the user might insert tags that disrupt this guarantee. However, assuming the user doesn't
manually write their own `<tags>` then there is this guarantee.

Tagger also provides functionality to build svg paths and polyline attribute data.

Sometimes, having to deal with all the borrowing and closures is difficult, though. For these cases
a traditional ElementStack can be used. This allows you to move around the stack between functions and
classes easily. The downside is that you lose compile time assurance that every push matches every pop.


### Example

```rust
use tagger::prelude::*;
fn main() -> core::fmt::Result {
    let width = 100.0;
    let height = 100.0;

    let mut root = tagger::Element::new(tagger::upgrade(std::io::stdout()));

    root.elem("svg", |header| {
        let svg = header.write(|b| {
            b.attr("xmlns", "http://www.w3.org/2000/svg")?
                .with_attr("viewBox", wr!("0 0 {} {}", width, height))
        })?;

        svg.single("rect", |w| {
            w.attr("x1", 0)?
                .attr("y1", 0)?
                .attr("rx", 20)?
                .attr("ry", 20)?
                .attr("width", width)?
                .attr("height", height)?
                .attr("style", "fill:blue")
        })?;

        //Add styling for test class.
        svg.elem_no_attr("style", |style| {
            write_ret!(style, "{}", ".test{fill:none;stroke:white;stroke-width:3}")
        })?;

        //Draw some circles
        svg.elem("g", |header| {
            let g = header.write(|w| w.attr("class", "test"))?;
            for r in (0..50).step_by(10) {
                g.single("circle", |w| {
                    w.attr("cx", 50.0)?.attr("cy", 50.0)?.attr("r", r)
                })?;
            }
            Ok(g)
        })
    })?;

    Ok(())
}

```




### Output


<img src="./assets/svg_example.svg" alt="demo">
