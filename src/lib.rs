//! This crate provides primitives to build up a html/xml/svg document programatically,
//! as opposed to a templating type engine.
//!
//! ### Why are these all macros?
//!
//! So that the user can pass as many format arguments as desired.
//!
//! ### Why do I have to call `end!()`?
//!
//! This is to force the user to handle the error case
//! of writing the end tag. If we did this in the destructor of
//! an element, then the write could silently fail.
//!
//! So we enforce that `end!()` was called at runtime by checking
//! a flag in the destructor and panic if it was not called.
//! If the destructor is called as part of unwinding, then it
//! does nothing.
//!
//!

///Contains primitives to make some svg constructs like paths and polylines.
pub mod svg;

///The prelude to import the element manipulation convenience macros.
pub mod prelude {
    pub use super::wr;
    pub use core::fmt::Write;
}


use core::fmt;



use fmt::Write;

#[macro_export]
macro_rules! wr {
    ($($arg:tt)*) => {
        move |w|write!(w,$($arg)*)
    }
}


pub fn single<T:Write>(w:&mut T,a:impl FnOnce(&mut T)->fmt::Result)->fmt::Result{
    a(w)
}
pub fn elem<'a,T:Write,F:FnOnce(&mut T)->fmt::Result>(
    writer:&'a mut T,func:impl FnOnce(&mut T)->fmt::Result,func2:F)->Result<Element<'a,T,F>,fmt::Error>{
    Element::new(writer,func,func2)
}
pub struct Element<'a,T,F>{
    writer:&'a mut T,
    func:Option<F>
}
impl<'a,T:Write,F:FnOnce(&mut T)->fmt::Result> Element<'a,T,F>{
    pub fn new(writer:&'a mut T,a:impl FnOnce(&mut T)->fmt::Result,func:F)->Result<Element<'a,T,F>,fmt::Error>{
        (a)(writer)?;
        Ok(Element{
            writer,
            func:Some(func)
        })
    }
    pub fn single(&mut self,a:impl FnOnce(&mut T)->fmt::Result)->fmt::Result{
        (a)(self.writer)
    }
    pub fn elem<'b,F1:FnOnce(&mut T)->fmt::Result>(&'b mut self,a:impl FnOnce(&mut T)->fmt::Result,func:F1)->Result<Element<'b,T,F1>,fmt::Error>{
        (a)(self.writer)?;
        Ok(Element{
            writer:self.writer,
            func:Some(func)
        })
    }
    pub fn end(mut self)->fmt::Result{
        (self.func.take().unwrap())(self.writer)
    }
}
impl<'a,T,F> Drop for Element<'a,T,F>{
    fn drop(&mut self){
        if !self.func.is_none() && !std::thread::panicking() {
            panic!("end() must be called on this element",);
        }
    }
}


///Used by [`upgrade`]
pub struct WriterAdaptor<T> {
    pub inner: T,
    pub error: Result<(), std::io::Error>,
}

///Upgrade a [`std::io::Write`] to be a [`std::fmt::Write`]
pub fn upgrade<T: std::io::Write>(inner: T) -> WriterAdaptor<T> {
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
