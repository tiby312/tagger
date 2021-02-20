use tagger::prelude::*;

fn main() -> core::fmt::Result {
    let width = 100.0;
    let height = 100.0;

    tagger::xml::xml(&mut tagger::upgrade(std::io::stdout()))?.defer_end(|root| {
        root.prolog("xml", |w| {
            w.attr("version", "1.0")?.attr("encoding", "UTF-8")
        })?;

        root.comment(wr!("hay"))?;

        let svg = root.elem("svg", |b| {
            b.attr("xmlns", "http://www.w3.org/2000/svg")?
                .with_attr("viewBox", wr!("0 0 {} {}", width, height))
        })?;

        svg.defer_end(|svg| {
            //Draw a blue background
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
            svg.elem_no_attr("style")?.defer_end(|style| {
                style.inner_str(".test{fill:none;stroke:white;stroke-width:3}")
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
            svg.elem("g", |w| w.attr("class", "test"))?.defer_end(|g| {
                for r in (0..50).step_by(10) {
                    g.single("circle", |w| {
                        w.attr("cx", 50.0)?.attr("cy", 50.0)?.attr("r", r)
                    })?;
                }
                Ok(())
            })
        })
    })
}
