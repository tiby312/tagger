//! This crate provides primitives to build up a html/xml/svg document programatically,
//! as opposed to a templating type engine.
//!
//! ### Why do I have to call `end()`?
//!
//! This is to force the user to handle the error case
//! of writing the end tag. If we did this in the destructor of
//! an element, then the write could silently fail.
//!
//! So we enforce that `end()` was called at runtime by checking
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
    pub use super::wrstr;
    pub use core::fmt::Write;
}

use core::fmt;

use fmt::Write;

///Convenience macro to reduce code.
///Create a closure that will use write!() with the formatting arguments.
#[macro_export]
macro_rules! wr {
    ($($arg:tt)*) => {
        move |w|write!(w,$($arg)*)
    }
}



///Convenience macro to reduce code.
///Create a closure that will use write!() with the formatting arguments.
#[macro_export]
macro_rules! wrstr {
    ($arg:tt) => {
        move |w|write!(w,"{}",$arg)
    }
}






pub struct TagBuilder<'a,T>{
    inner:&'a mut T
}
impl<'a,T:Write> TagBuilder<'a,T>{
    pub fn new(inner:&'a mut T,tag:&str)->Result<TagBuilder<'a,T>,core::fmt::Error>{
        write!(inner,"<{}",tag)?;
        Ok(TagBuilder{inner})
    }

    pub fn with_attr(self,s:&str,func:impl FnOnce(&mut T)->core::fmt::Result)->Result<TagBuilder<'a,T>,core::fmt::Error>{
        write!(self.inner," {}=",s)?;
        write!(self.inner,"\"")?;
        func(self.inner)?;
        write!(self.inner,"\"")?;
        Ok(self)
    }
    pub fn attr(self,s:&str,val:impl core::fmt::Display)->Result<TagBuilder<'a,T>,core::fmt::Error>{
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
    fn finish_prolog(self)->core::fmt::Result{
        write!(self.inner,"?>")?;
        Ok(())
    }
}


pub fn xml<'a,T:Write>(writer:&'a mut T)->Result<XML<'a,T,impl FnOnce(&mut T)->fmt::Result+'a>,fmt::Error>{
    Ok(XML{
        inner:Element::new(writer,
            move|_|Ok(()),
            move|_  |Ok(())
        )?
    })
}
pub struct XML<'a,T,F>{
    inner:Element<'a,T,F>
}
impl<'a,T:Write,F: FnOnce(&mut T) -> fmt::Result> XML<'a,T,F>{
    pub fn single<'b>(&'b mut self,tag:&'b str,func:impl FnOnce(TagBuilder<T>)->Result<TagBuilder<T>,fmt::Error>+'b)->fmt::Result
    {
        func(TagBuilder::new(self.inner.writer,tag)?)?.finish_single()
    }

    pub fn move_inner(self,func:impl FnOnce(&mut T)->fmt::Result)->Result<Self,fmt::Error>
    {
        func(self.inner.writer)?;
        Ok(self)
    }
    pub fn inner(&mut self,func:impl FnOnce(&mut T)->fmt::Result)->fmt::Result
    {
        func(self.inner.writer)
    }
    pub fn inner_str(&mut self,s:&str)->fmt::Result
    {
        write!(self.inner.writer,"{}",s)
    }

    
    pub fn declaration(&mut self,tag:&str,func:impl FnOnce(&mut T)->fmt::Result)->fmt::Result{
        let w=&mut self.inner.writer;
        write!(w,"<!{} ",tag)?;
        func(w)?;
        write!(w,">")?;
        Ok(())
    }
    pub fn prolog(&mut self,tag:&str,func:impl FnOnce(TagBuilder<T>)->Result<TagBuilder<T>,fmt::Error>)->fmt::Result{
        let w=&mut self.inner.writer;
        write!(w,"<?{}",tag)?;
        func(TagBuilder{inner:w})?.finish_prolog()?;
        Ok(())
    }
    
    pub fn comment(&mut self,func:impl FnOnce(&mut T)->fmt::Result)->fmt::Result{
        let w=&mut self.inner.writer;
        write!(w,"<!--")?;
        func(w)?;
        write!(w," -->")
    }

    pub fn elem_simple<'b>(&'b mut self,tag:&'b str)->Result<XML<'b,T,impl FnOnce(&mut T)->fmt::Result+'b>,fmt::Error>{
        self.elem(tag,|w|Ok(w))   
    }
    pub fn elem<'b>(&'b mut self,tag:&'b str,func:impl FnOnce(TagBuilder<T>)->Result<TagBuilder<T>,fmt::Error>+'b)->
        Result<XML<'b,T,impl FnOnce(&mut T)->fmt::Result+'b>,fmt::Error>{
        Ok(XML{
            inner:self.inner.elem(move|w|func(TagBuilder::new(w,tag)?)?.finish(),move|w|write!(w,"</{}>",tag) )?
        })
    }
    pub fn end(self)->fmt::Result{
        self.inner.end()
    }
}


pub fn json<'a,T:Write>(writer:&'a mut T)->Result<JSON<'a,T,impl FnOnce(&mut T)->fmt::Result+'a>,fmt::Error>{
    Ok(
        JSON{
            inner:Element::new(
                    writer,
                    move|w| {
                        write!(w,"{{")
                    },
                    move|w|write!(w,"}}")
                    )?,
            atleast_one_attr:false
        }
    )
}
pub struct JSON<'a,T,F>{
    inner:Element<'a,T,F>,
    atleast_one_attr:bool
}
impl<'a,T: Write, F: FnOnce(&mut T) -> fmt::Result> JSON<'a,T,F>{

    pub fn elem<'b>(&'b mut self,tag:&'b str)->Result<JSON<'b,T,impl FnOnce(&mut T)->fmt::Result+'b>,fmt::Error>{
        let atleast_one_attr=self.atleast_one_attr;
        Ok(JSON{
            inner:self.inner.elem(move|w|{
                    if atleast_one_attr {
                        write!(w,",\"{}\":{{",tag)?
                    }else{
                        write!(w,"\"{}\":{{",tag)?
                    } 
                    Ok(())
                } ,move|w|write!(w,"}}"))?,
            atleast_one_attr:false
        })
    }

    pub fn inner(&mut self,s:&str,f:impl core::fmt::Display)->fmt::Result{
        if self.atleast_one_attr {
            write!(self.inner.writer,",\"{}\":\"{}\"",s,f)?
        }else{
            write!(self.inner.writer,"\"{}\":\"{}\"",s,f)?
        }
        self.atleast_one_attr=true;
        Ok(())
    }

    pub fn end(self)->fmt::Result{
        self.inner.end()
    }
}


///The base element structure.
///It will panic if the user doesnt properly
///call end() on it.
pub struct Element<'a, T, F> {
    writer: &'a mut T,
    func: Option<F>,
}
impl<'a, T: Write, F: FnOnce(&mut T) -> fmt::Result> Element<'a, T, F> {
    ///Write an element.
    pub fn new(
        writer: &'a mut T,
        a: impl FnOnce(&mut T) -> fmt::Result,
        func: F,
    ) -> Result<Element<'a, T, F>, fmt::Error> {
        (a)(writer)?;
        Ok(Element {
            writer,
            func: Some(func)
        })
    }

    ///Write an element with no end tag.
    pub fn single(&mut self, a: impl FnOnce(&mut T) -> fmt::Result) -> fmt::Result {
        (a)(self.writer)
    }

    ///Start a new element.
    pub fn elem_tuple<'b, F0:FnOnce(&mut T) -> fmt::Result, F1: FnOnce(&mut T) -> fmt::Result>(
        &'b mut self,
        a: (F0,F1),
    ) -> Result<Element<'b, T, F1>, fmt::Error> {
        (a.0)(self.writer)?;
        Ok(Element {
            writer: self.writer,
            func: Some(a.1)
        })
    }

    ///Start a new element.
    pub fn elem<'b, F1: FnOnce(&mut T) -> fmt::Result>(
        &'b mut self,
        a: impl FnOnce(&mut T) -> fmt::Result,
        func: F1,
    ) -> Result<Element<'b, T, F1>, fmt::Error> {
        (a)(self.writer)?;
        Ok(Element {
            writer: self.writer,
            func: Some(func)
        })
    }

    ///End the current element.
    pub fn end(mut self) -> fmt::Result {
        (self.func.take().unwrap())(self.writer)
    }
}


impl<'a, T, F> Drop for Element<'a, T, F> {
    fn drop(&mut self) {
        if !self.func.is_none() && !std::thread::panicking() {
            panic!("end() was not called on this element",);
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
