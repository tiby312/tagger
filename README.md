### Overview

Build xml / html / svg programatically using element building blocks.
Instead of using a templating engine, write data/markup that 'looks like' rust.

Find it on [github](https://github.com/tiby312/tagger) and [crates.io](https://crates.io/crates/tagger).

Tagger also provides functionality to build svg paths and polyline attribute data.

### Example

```rust
use tagger::prelude::*;
use tagger::attr_builder;

fn main() {
    let width = 100.0;
    let height = 100.0;

    let mut svg = elem!(
        "svg",
        attr_builder()
            .attr("xmlns", "http://www.w3.org/2000/svg")
            .attr("viewBox", formatm!("0 0 {} {}", width, height))
            .build()
    );

    let rect = {
        let rect_attr = attr_builder()
            .attr("x1", 0)
            .attr("y1", 0)
            .attr("rx", 20)
            .attr("ry", 20)
            .attr("width", width)
            .attr("height", height)
            .attr("style", "fill:blue")
            .build();
        single!("rect", rect_attr)
    };

    svg.append(rect);

    let style = elem!("style").add(single!(".test{fill:none;stroke:white;stroke-width:3}"));
    svg.append(style);

    let g = {
        let gc = attr_builder().attr("class", "test").build();
        let mut g = elem!("g", gc);
        for r in (0..50).step_by(10) {
            g.append(single!(
                "circle",
                attr_builder()
                    .attr("cx", 50.0)
                    .attr("cy", 50.0)
                    .attr("r", r)
                    .build()
            ));
        }
        g
    };

    svg.append(g);

    println!("{}", svg);
}

```




### Output


<img src="./assets/svg_example.svg" alt="demo">
