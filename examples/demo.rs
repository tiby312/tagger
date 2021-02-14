
use core::fmt::Error;
fn main()->Result<(),Error>{
    
    let mut string=String::new();

    {
        let mut el=tagger::Element::new(&mut string,"start").tend();
        
        let mut html=el.tag("html").app("DOC_TYPE").tend();
        html.tag("rect").app("class='poloto2fill' height='7.5' rx='5' ry='5' width='50' x='680' y='176.25'").tcut();
        
        let mut style=html.tag("style").tend();
        style.write_str(".potato{chicken}\n");
        drop(style);
        
        let mut div=html.tag("div").app("x=5").tend();
        div.tag("svg").app("foo").tend();
        div.tag("svg").app("blag").tend();
        div.tag("img").app("width='100%'").tcut();
        drop(div);


        html.tag("div").app("kiki=7").tend();
        
    }
    use std::io::Write;
    std::io::stdout().write_all(&string.as_bytes()).unwrap();
    Ok(())
}