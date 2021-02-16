
pub use inner::Element;
pub use inner::Single;
mod inner{
    use core::fmt::Write;
    use core::fmt;

    pub struct Single<'a,T:Write>{
        writer:&'a mut T
    }
    impl<'a,T:Write> Single<'a,T>{
        pub fn borrow<'b>(&'b mut self,a:fmt::Arguments<'_>,b:&'b str)->Result<Element<'b,T>,fmt::Error>{
            self.writer.write_fmt(a)?;
            Ok(Element{
                writer:self.writer,
                func:Some(b)
            })
        }
    }
    pub struct Element<'a,T:Write>{
        writer:&'a mut T,
        func:Option<&'a str>
    }
    impl<'a,T:Write> Drop for Element<'a,T>{
        fn drop(&mut self){
            
            let _ = self.writer.write_str(self.func.take().unwrap());
        }
    }
    impl<'a,T:Write> Element<'a,T>{
        pub fn new(writer:&'a mut T,ar:fmt::Arguments,func2:&'a str)->Result<Self,fmt::Error>{
            writer.write_fmt(ar)?;
            Ok(Element{
                writer,
                func:Some(func2)
            })
        }
        pub fn borrow<'b>(&'b mut self,a:fmt::Arguments,b:&'a str)->Result<Element<'b,T>,fmt::Error>{
            self.writer.write_fmt(a)?;
            Ok(Element{
                writer:self.writer,
                func:Some(b)
            })
        }
        pub fn borrow_single<'b>(&'b mut self,a:fmt::Arguments)->Result<Single<'b,T>,fmt::Error>{
            self.writer.write_fmt(a)?;
            Ok(Single{
                writer:self.writer
            })
        }
    }
}

#[macro_export]
macro_rules! new_element {
    ($dst:expr,$arg1:expr, $tail:expr) => {
        crate::Element::new($dst,format_args!($arg1),$tail)
    };
    ($dst:expr,$arg1:expr, $tail:expr,$($arg:tt)*) => {
        crate::Element::new($dst,format_args!($arg1,$($arg)*),$tail)
    }
}
#[macro_export]
macro_rules! element {
    ($dst:expr,$arg1:expr, $tail:expr) => {
        $dst.borrow(format_args!($arg1),$tail)
    };
    ($dst:expr,$arg1:expr, $tail:expr,$($arg:tt)*) => {
        $dst.borrow(format_args!($arg1,$($arg)*),$tail)
    }
}
#[macro_export]
macro_rules! empty_element {
    ($dst:expr,$arg1:expr) => {
        $dst.borrow_single(format_args!($arg1))
    };
    ($dst:expr,$arg1:expr,$($arg:tt)*) => {
        $dst.borrow_single(format_args!($arg1,$($arg)*))
    }
}


use core::fmt;


pub fn poly(a:impl ExactSizeIterator<Item=[f32;2]>)->impl fmt::Display{
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
                write!(f,"{},{} ",x,y)?
            }
            Ok(())
        }
    }
    PolyLine::new(a)
}



///Used by [`upgrade_writer`]
pub struct WriterAdaptor<T> {
    pub inner: T,
    pub error: Result<(), std::io::Error>,
}

///Upgrade a std::io::Write to be a std::fmt::Write
pub fn upgrade_writer<T: std::io::Write>(inner: T) -> WriterAdaptor<T> {
    WriterAdaptor {
        inner,
        error: Ok(()),
    }
}
impl<T: std::io::Write> fmt::Write for WriterAdaptor<T> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        match self.inner.write_all(s.as_bytes()) {
            Ok(()) => Ok(()),
            Err(e) => {
                self.error = Err(e);
                Err(fmt::Error)
            }
        }
    }
}


