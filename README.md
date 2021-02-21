### Overview

Build xml / html / svg programatically using element building blocks.
Instead of using a templating engine, write data/markup that 'looks like' rust.

Find it on [github](https://github.com/tiby312/tagger) and [crates.io](https://crates.io/crates/tagger).

### Example

```rust

use tagger::prelude::*;
use tagger::tag_types;
fn main() -> core::fmt::Result {
    let width = 100.0;
    let height = 100.0;

    let mut root = tagger::Element::new(tagger::upgrade(std::io::stdout()));
    root.single_ext("xml", tag_types::PROLOG, |a| {
        a.with_attr("version", wr!("1.0"))?
            .attr("encoding", "UTF-8")?
            .attr("standalone", "yes")
    })?;

    root.single_ext("", tag_types::COMMENT, |a| {
        write!(a, "{}", "This is a comment")?;
        Ok(a)
    })?;

    root.elem("svg", |b| {
        let (svg, cert) = b.build(|b| {
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
            write!(style, "{}", ".test{fill:none;stroke:white;stroke-width:3}")
        })?;

        //Draw a poly line
        svg.single("polyline", |w| {
            w.attr("style", "fill:none;stroke:red")?;
            w.polyline_data(|p| {
                for i in 0..100 {
                    p.add_point([i as f32, (((i as f32) * 10.0 / 100.0).sin() + 1.0) * 25.0])?;
                }
                Ok(p)
            })
        })?;

        //Draw a path
        svg.single("path", |w| {
            w.attr("style", "fill:none;stroke:green")?;
            w.path_data(|p| {
                p.move_to([50, 50])?;
                for i in 0..100 {
                    p.line_to([i as f32, (((i as f32) * 10.0 / 100.0).cos() + 1.0) * 25.0])?;
                }
                p.close()
            })
        })?;

        //Draw some circles
        svg.elem("g", |builder| {
            let (g, cert) = builder.build(|w| w.attr("class", "test"))?;
            for r in (0..50).step_by(10) {
                g.single("circle", |w| {
                    w.attr("cx", 50.0)?.attr("cy", 50.0)?.attr("r", r)
                })?;
            }
            cert
        })?;

        cert
    })
}

```




### Output


<img src="./assets/svg_example.svg" alt="demo">
