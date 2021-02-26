use tagger::prelude::*;
use tagger::svg::PathCommand;
        
fn main() -> core::fmt::Result {
    let width = 500.0;
    let height = 400.0;

    let mut root = tagger::Element::new(tagger::upgrade(std::io::stdout()));

    root.elem("svg", |header| {
        let svg = header.write(|b| {
            b.attr("xmlns", "http://www.w3.org/2000/svg")?
                .with_attr("viewBox", wr!("0 0 {} {}", width, height))
        })?;



        //Draw a path
        svg.single("path", |w| {
            w.attr("stroke","black")?;
            w.attr("stroke-width",2)?;
            w.attr("fill","green")?;
            w.attr("fill-opacity",0.5)?;
            use PathCommand::*;

            w.path_data(|p|{
                p.draw(M(100,200))?;
                p.draw(C(100,100,250,100,250,200))?;
                p.draw(S(400,300,400,200))
                
                
            })
        })?;

        svg.single("path", |w| {
            w.attr("stroke","black")?;
            w.attr("stroke-width",2)?;
            w.attr("fill","red")?;
            w.attr("fill-opacity",0.5)?;
            use PathCommand::*;

            w.path_data(|p|{
                
                p.draw(M(200,120))?;
                p.draw(Q(300,50,400,120))?;
                p.draw(T(500,120))
                
            })
        })?;

        svg.single("path", |w| {
            w.attr("stroke","black")?;
            w.attr("stroke-width",2)?;
            w.attr("fill","blue")?;
            w.attr("fill-opacity",0.5)?;
            use PathCommand::*;

            w.path_data(|p|{
                

                p.draw(M(300,200))?;
                p.draw(H_(-150))?;
                p.draw(A_(150,150,0,1,0,150,-150))?;
                p.draw_z()
            })
        })
    })?;

    Ok(())
}
