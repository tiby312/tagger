use tagger::prelude::*;
use tagger::svg::PathCommand;
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
        write_ret!(a, "{}", "This is a comment")
    })?;

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

        //Draw a poly line
        svg.single("polyline", |w| {
            w.attr("style", "fill:none;stroke:red")?;
            w.points_data(|p| {
                for i in 0..100 {
                    p.add_point(i as f32, (((i as f32) * 10.0 / 100.0).sin() + 1.0) * 25.0)?;
                }
                Ok(p)
            })
        })?;

        //Draw a path
        svg.single("path", |w| {
            w.attr("style", "fill:none;stroke:green")?;
            w.path_data(|p| {
                p.draw(PathCommand::M(50, 50))?;
                for i in 0..100 {
                    let x = i as f32;
                    let y = (((i as f32) * 10.0 / 100.0).cos() + 1.0) * 25.0;
                    p.draw(PathCommand::L(x, y))?;
                }
                p.draw_z()
            })
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
