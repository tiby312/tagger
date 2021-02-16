use tagger::prelude::*;

mod foo{
    use core::fmt::Write;
    use core::fmt;

    pub struct Single<'a,T:Write>{
        writer:&'a mut T
    }
    impl<'a,T:Write> Single<'a,T>{
        pub fn borrow<'b>(&'b mut self,a:fmt::Arguments<'_>,b:&'b str)->Floop<'b,T>{
            self.writer.write_fmt(a);
            Floop{
                writer:self.writer,
                func:Some(b)
            }
        }
    }
    pub struct Floop<'a,T:Write>{
        writer:&'a mut T,
        func:Option<&'a str>
    }
    impl<'a,T:Write> Drop for Floop<'a,T>{
        fn drop(&mut self){
            
            let _ = self.writer.write_str(self.func.take().unwrap());
        }
    }
    impl<'a,T:Write> Floop<'a,T>{
        pub fn new(writer:&'a mut T,ar:fmt::Arguments,func2:&'a str)->Self{
            writer.write_fmt(ar);
            Floop{
                writer,
                func:Some(func2)
            }
        }
        pub fn borrow<'b>(&'b mut self,a:fmt::Arguments,b:&'a str)->Floop<'b,T>{
            self.writer.write_fmt(a);
            Floop{
                writer:self.writer,
                func:Some(b)
            }
        }
        pub fn borrow_single<'b>(&'b mut self,a:fmt::Arguments)->Single<'b,T>{
            self.writer.write_fmt(a);
            Single{
                writer:self.writer
            }
        }
    }
}


macro_rules! new_element {
    ($dst:expr,$arg1:expr, $tail:expr) => {
        foo::Floop::new($dst,format_args!($arg1),$tail)
    };
    ($dst:expr,$arg1:expr, $tail:expr,$($arg:tt)*) => {
        foo::Floop::new($dst,format_args!($arg1,$($arg)*),$tail)
    }
}

macro_rules! element {
    ($dst:expr,$arg1:expr, $tail:expr) => {
        $dst.borrow(format_args!($arg1),$tail)
    };
    ($dst:expr,$arg1:expr, $tail:expr,$($arg:tt)*) => {
        $dst.borrow(format_args!($arg1,$($arg)*),$tail)
    }
}
macro_rules! element_empty {
    ($dst:expr,$arg1:expr) => {
        $dst.borrow_single(format_args!($arg1))
    };
    ($dst:expr,$arg1:expr,$($arg:tt)*) => {
        $dst.borrow_single(format_args!($arg1,$($arg)*))
    }
}


use core::fmt;


fn poly(a:impl ExactSizeIterator<Item=[f32;2]>)->impl fmt::Display{
    struct PolyLine<I>{
        it:std::cell::RefCell<I>
    }
    impl<I:Iterator<Item=[f32;2]>> PolyLine<I>{
        fn new(it:I)->PolyLine<I>{
            PolyLine{
                it:std::cell::RefCell::new(it)
            }
        }
    }
    impl<I:Iterator<Item=[f32;2]>> fmt::Display for PolyLine<I>{
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            for [x,y] in &mut *self.it.borrow_mut(){
                write!(f,"{} {},",x,y)?
            }
            Ok(())
        }
    }
    PolyLine::new(a)
}




//TODO make path data / polyline data implement Display.
//THEN you can pass it to the formatter!!!!

fn main() {
    let mut string = String::new();
    {
        use core::fmt::Write;
        
        let mut k=new_element!(&mut string,"<rect x={},y={}>\n","</rect>",4,5);
        
        let mut p=poly((0..5).map(|x|[x as f32,x as f32]));

        let mut j=element!(k,"<svg x={} y={} d={}>","</svg>",5,4,p);
        element_empty!(j,"<chicken>");

        //j.borrow_single(format_args!("<img x={}/>\n",4));

        //let a=root!(k,"<rect={}>",4,"</rect>");
        //let j=element!(a,"<img x={}>",4,"</img>");
        //single!(j,"<img x={}>",4);



        //k.borrow(|w|write!(w,"<img x={}>\n",4),|w|write!(w,"</img>\n"));
        
    }


    {
        let mut root = tagger::root(&mut string);
        root.declaration("DOCTYPE html");

        let mut html = root.tag_build("html").end();

        //html.tag_build("rect").append("class='poloto2fill' height='7.5' rx='5' ry='5' width='50' x='680' y='176.25'").empty();

        html.tag_build("rect").set("width", 4).empty_no_slash();
        html.comment("test comment!");

        html.tag_build("rect")
            .set("class", "poloto2fill")
            .set("height", 7.5)
            .set("rx", 5)
            .empty();

        let mut style = html.tag_build("style").end();
        style.write_str(".potato{chicken}\n");
        drop(style);

        let mut div = html.tag_build("div").append("x=5").end();
        div.tag_build("svg").append("foo").end();
        div.tag_build("svg").append("blag").end();
        div.tag_build("img").append("width='100%'").empty();
        drop(div);

        html.tag_build("div").append("kiki=7").end();
    }
    use std::io::Write;
    std::io::stdout().write_all(&string.as_bytes()).unwrap();
}
