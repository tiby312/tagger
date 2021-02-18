use tagger::prelude::*;


struct JsonBuilder<'a,T>{
    inner:&'a mut T
}

impl<'a,T> JsonBuilder<'a,T>{
    fn new(inner:&'a mut T,tag:&str)->Result<JsonBuilder<'a,T>,core::fmt::Error>{
        unimplemented!();
    }
}
struct TagBuilder<'a,T>{
    inner:&'a mut T
}
impl<'a,T:Write> TagBuilder<'a,T>{
    fn new(inner:&'a mut T,tag:&str)->Result<TagBuilder<'a,T>,core::fmt::Error>{
        write!(inner,"<{}",tag)?;
        Ok(TagBuilder{inner})
    }

    fn with_attr(self,s:&str,func:impl FnOnce(&mut T)->core::fmt::Result)->Result<TagBuilder<'a,T>,core::fmt::Error>{
        write!(self.inner," {}=",s)?;
        write!(self.inner,"\"")?;
        func(self.inner)?;
        write!(self.inner,"\"")?;
        Ok(self)
    }
    fn attr(self,s:&str,val:impl core::fmt::Display)->Result<TagBuilder<'a,T>,core::fmt::Error>{
        write!(self.inner," {}=\"{}\"",s,val)?;
        Ok(self)
    }

    fn finish(self)->core::fmt::Result{
        write!(self.inner,">")?;
        Ok(())
    }
    fn finish_single(self)->core::fmt::Result{
        write!(self.inner,"/>")?;
        Ok(())
    }
}

use core::fmt;
//TODO use this!!! AMAZING!!!!
fn flop<'a,T:Write>(tag:&'a str,func:impl FnOnce(TagBuilder<T>)->core::fmt::Result+'a)->
    (impl FnOnce(&mut T)->fmt::Result+'a,impl FnOnce(&mut T)->fmt::Result+'a){
    (move |w|{
        let mut t=TagBuilder::new(w,tag)?;
        func(t)
    },
    move |w|write!(w,"</{}>",tag)
    )
}

fn main() -> core::fmt::Result {
    let mut io = tagger::upgrade(std::io::stdout());
    let width = 100.0;
    let height = 100.0;

    let mut svg=tagger::elem(
        &mut io,
        |w|TagBuilder::new(w,"svg")?
        .attr("xmlns","http://www.w3.org/2000/svg")?
        .with_attr("viewBox",|w|write!(w,"0 0 {} {}",width,height))?
        .finish(),
        wr!("</svg>")
    )?;
    
    svg.single(wr!(
        "{}",
        "<style>.test{fill:none;stroke:white;stroke-width:3}</style>"
    ))?;

    let c=svg.elem_tuple(flop("chicken",|w|w.attr("width",width)?.finish()))?;

    c.end()?;
    svg.single(|w|
        TagBuilder::new(w,"rect")?
        .attr("width",width)?
        .attr("height",height)?
        .attr("rx",20)?
        .attr("ry",20)?
        .attr("style","fill:blue;")?
        .finish_single()
    )?;
    /*
    svg.single(wr!(
        "<rect width='{}' height='{}' rx='{}' ry='{}' style='fill:blue;'/>",
        width,
        height,
        20,
        20
    ))?;
    */
    let mut g = svg.elem(wr!("<g class='test'>"), wr!("</g>"))?;
    for r in (0..50).step_by(10) {
        g.single(wr!("<circle cx='{}' cy='{}' r='{}'/>", 50.0, 50.0, r))?;
    }

    //Program panics if the elements aren't closed.
    g.end()?;
    svg.end()?;
    Ok(())
}
