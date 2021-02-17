Build xml/html/svg programatically using element building blocks.


### Example

```rust
use tagger::prelude::*;

fn main() -> core::fmt::Result {
    let mut io = tagger::upgrade(std::io::stdout());
    let width = 100.0;
    let height = 100.0;

    let mut svg = new_element!(
        &mut io,
        "<svg viewBox='0 0 {} {}' xmlns='http://www.w3.org/2000/svg'>",
        width,
        height
    )?;

    empty_element!(
        svg,
        "<style>.test{{fill:none;stroke:white;stroke-width:3}}</style>"
    )?;

    empty_element!(
        svg,
        "<rect width='{}' height='{}' rx='{}' yx='{}' style='fill:blue;'/>",
        width,
        height,
        20,
        20
    )?;

    let mut g = element!(svg, "<g class='test'>")?;
    for r in (0..50).step_by(10) {
        empty_element!(g, "<circle cx='{}' cy='{}' r='{}'/>", 50.0, 50.0, r)?;
    }
    end!(g, "</g>")?;

    end!(svg, "</svg>")?;

    Ok(())
}
```




### Output


<img src="./assets/svg_example.svg" alt="demo">
