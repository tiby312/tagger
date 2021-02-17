use tagger::prelude::*;

fn main() -> core::fmt::Result {
    let io = tagger::upgrade(std::io::stdout());
    let width = 100.0;
    let height = 100.0;

    let mut stack = tagger::elem::ElementStack::new(
        io,
        format_args!(
            "<svg viewBox='0 0 {} {}' xmlns='http://www.w3.org/2000/svg'>",
            width, height
        ),
        "</svg>",
    )?;

    stack.single(format_args!(
        "{}",
        "<style>.test{fill:none;stroke:white;stroke-width:3}</style>"
    ))?;

    stack.single(format_args!(
        "<rect width='{}' height='{}' rx='{}' yx='{}' style='fill:blue;'/>",
        width, height, 20, 20
    ))?;

    stack.borrow_move(format_args!("{}", "<g class='test'>"), "</g>")?;

    for r in (0..50).step_by(10) {
        stack.single(format_args!(
            "<circle cx='{}' cy='{}' r='{}'/>",
            50.0, 50.0, r
        ))?;
    }

    //pop g
    stack.end_last()?;

    //pop svg
    stack.end_last()?;

    Ok(())
}
