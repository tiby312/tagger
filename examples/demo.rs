

use tagger::*;
use core::fmt;
//TODO make path data / polyline data implement Display.
//THEN you can pass it to the formatter!!!!

fn main()->Result<(),fmt::Error> {
    let mut string = String::new();
    {
        
        let mut k=new_element!(&mut string,"<rect x={},y={}>","</rect>",4,5)?;
        
        let p=poly((0..5).map(|x|[x as f32,x as f32]));

        
        let mut j=element!(k,"<svg x={} y={} points={polydata}>","</svg>",5,4,polydata=p)?;
        empty_element!(j,"<chicken>")?;

        
    }
    println!("{}",string);

    Ok(())
}
