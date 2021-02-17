

use tagger::*;


fn main()->core::fmt::Result{
    let mut io=tagger::upgrade(std::io::stdout());
    let width=100.0;
    let height=100.0;
    
    let mut svg=new_element!(&mut io,"<svg viewBox='0 0 {} {}' xmlns='http://www.w3.org/2000/svg'>",width,height)?;

    let mut g=element!(svg,"<g>")?;

    empty_element!(g,"<circle cx='{}' cy='{}' r='{}'/>",50.0,50.0,50.0)?;

    end!(g,"</g>")?;
    
    
    end!(svg,"</svg>")?;


    Ok(())
}