use tagger::prelude::*;

fn main()->core::fmt::Result{
    let mut io=tagger::upgrade(std::io::stdout());
    
    let mut root=new_empty_element!(&mut io,"<!DOCTYPE html>")?;
    let mut html=element!(root,"<html>")?;
    
    empty_element!(html,"<style>{}</style>",
    "table, th, td {
      border: 1px solid black;
      border-collapse: collapse;
      animation: mymove 5s infinite;
    }
    @keyframes mymove {
        from {background-color: red;}
        to {background-color: blue;}
    }"
    )?;

    let mut table=element!(html,"<table style='width:{}%'>",100)?;
    
    for i in 0..20{
        let mut tr=element!(table,"<tr>")?;

        empty_element!(tr,"<th>Hay {}:1</th>",i)?;
        empty_element!(tr,"<th>Hay {}:2</th>",i)?;
        empty_element!(tr,"<th>Hay {}:3</th>",i)?;
        
        end!(tr,"</tr>")?;
    }
    

    end!(table,"</table>")?;
    end!(html,"</html>")?;

    Ok(())
}