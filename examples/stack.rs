use tagger::prelude::*;
fn main() -> core::fmt::Result {
    let width = 100.0;
    let height = 100.0;

    let mut stack = tagger::ElementStack::new(tagger::upgrade(std::io::stdout()));

    stack.elem_stack("svg", |b| {
        b.attr("xmlns", "http://www.w3.org/2000/svg")?
            .with_attr("viewBox", wr!("0 0 {} {}", width, height))
    })?;

    stack.single("rect", |w| {
        w.attr("x1", 0)?
            .attr("y1", 0)?
            .attr("rx", 20)?
            .attr("ry", 20)?
            .attr("width", width)?
            .attr("height", height)?
            .attr("style", "fill:blue")
    })?;

    //Add styling for test class.
    stack.elem_no_attr("style", |style| {
        write_ret!(style, "{}", ".test{fill:none;stroke:white;stroke-width:3}")
    })?;

    stack.elem_stack("g", |w| w.attr("class", "test"))?;

    //Draw some circles
    for r in (0..50).step_by(10) {
        stack.single("circle", |w| {
            w.attr("cx", 50.0)?.attr("cy", 50.0)?.attr("r", r)
        })?;
    }

    stack.pop()?;

    stack.pop()?;

    stack.check_unwound();

    Ok(())
}
