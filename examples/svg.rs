use tagger::prelude::*;



fn main() -> core::fmt::Result {
    let mut io = tagger::upgrade(std::io::stdout());
    let width = 100.0;
    let height = 100.0;

    let mut root=tagger::xml(&mut io)?;

    root.prolog("xml",|w|w.attr("version","1.0")?.attr("encoding","UTF-8"))?;
    root.comment(wr!("hay"))?;
    
    let mut svg=root.elem("svg",
        |b|b.attr("xmlns","http://www.w3.org/2000/svg")?
        .with_attr("viewBox",wr!("0 0 {} {}",width,height))
    )?;

    
    let mut style=svg.elem_simple("style")?;
    style.inner_str(".test{fill:none;stroke:white;stroke-width:3}")?;
    style.end()?;

    svg.single("rect",|w|{

        for i in 0..10{
            w.attr("rx",i)?;
        }

        w
        .attr("width",width)?
        .attr("height",height)?;

        w.attr("rx",20)?
        .attr("ry",20)?
        .attr("style","fill:blue;")
    })?;

    /*
    svg.single("path",|w|
        .attr("class","poloto")?
        .path()?
        .move_to(5,2)?
        .move_to(6,3)?
        .move_to(3,3)?
        .close()?
        .end_path()?
        .attr("style","chicken")
    )?;
    */

    let mut g=svg.elem("g",|w|w.attr("class","test"))?;

    for r in (0..50).step_by(10) {
        g.single("circle",|w|w.attr("cx",50.0)?.attr("cy",50.0)?.attr("r",r))?;
    }

    g.end()?;

    svg.end()?;

    root.end()?;
    
    Ok(())
}
