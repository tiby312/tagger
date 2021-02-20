use super::*;
use crate::raw::Element;
use svg::*;

pub mod svg;

#[repr(transparent)]
pub struct AttrBuilder<'a, T> {
    inner: &'a mut T,
}
impl<'a, T: Write> AttrBuilder<'a, T> {
    pub fn new(inner: &'a mut T) -> AttrBuilder<'a, T> {
        AttrBuilder { inner }
    }

    pub fn polyline_data<'b,F>(&'b mut self,func:F ) -> Result<&'b mut AttrBuilder<'a,T>, fmt::Error> 
        where for<'x,'y> F:FnOnce(&'x mut PolyLineBuilder<'y,'a,T>)->Result<&'x mut PolyLineBuilder<'y, 'a, T>, fmt::Error>{
        {
            let mut p=PolyLineBuilder::new(self)?;
            func(&mut p)?;
            p.finish()?;
        }
        Ok(self)
    }


    pub fn path_data<'b,F>(&'b mut self,func:F ) -> Result<&'b mut AttrBuilder<'a,T>, fmt::Error> 
        where for<'x,'y> F:FnOnce(&'x mut PathBuilder<'y,'a,T>)->Result<&'x mut PathBuilder<'y, 'a, T>, fmt::Error>{
        {
            let mut p=PathBuilder::new(self)?;
            func(&mut p)?;
            p.finish()?;
        }
        Ok(self)
    }


    pub fn with_attr(
        &mut self,
        s: &str,
        func: impl FnOnce(&mut T) -> core::fmt::Result,
    ) -> Result<&mut Self, core::fmt::Error> {
        write!(self.inner, " {}=", s)?;
        write!(self.inner, "\"")?;
        func(&mut self.inner)?;
        write!(self.inner, "\"")?;
        Ok(self)
    }
    pub fn attr(
        &mut self,
        s: &str,
        val: impl core::fmt::Display,
    ) -> Result<&mut Self, core::fmt::Error> {
        write!(self.inner, " {}=\"{}\"", s, val)?;
        Ok(self)
    }
}

pub fn xml<'a, T: Write>(
    writer: &'a mut T,
) -> Result<XML<'a, T, impl FnOnce(&mut T) -> fmt::Result + 'a>, fmt::Error> {
    Ok(XML {
        inner: Element::new(writer, move |_| Ok(())),
    })
}

pub struct XML<'a, T, F> {
    inner: Element<'a, T, F>,
}
impl<'a, T: Write, F: FnOnce(&mut T) -> fmt::Result> XML<'a, T, F> {
    pub fn single<'b, FF>(&'b mut self, tag: &'b str, func: FF) -> fmt::Result
    where
        for<'x, 'y> FF:
            FnOnce(&'x mut AttrBuilder<'y, T>) -> Result<&'x mut AttrBuilder<'y, T>, fmt::Error>,
    {
        //THese are amazing!!!
        //https://doc.rust-lang.org/nomicon/hrtb.html

        write!(self.inner.writer, "<{} ", tag)?;
        let mut k = AttrBuilder::new(self.inner.writer);
        func(&mut k)?;

        write!(self.inner.writer, "/>")
    }

    pub fn move_inner(self, func: impl FnOnce(&mut T) -> fmt::Result) -> Result<Self, fmt::Error> {
        func(self.inner.writer)?;
        Ok(self)
    }
    pub fn inner(&mut self, func: impl FnOnce(&mut T) -> fmt::Result) -> fmt::Result {
        func(self.inner.writer)
    }
    pub fn inner_str(&mut self, s: &str) -> fmt::Result {
        write!(self.inner.writer, "{}", s)
    }

    pub fn declaration(
        &mut self,
        tag: &str,
        func: impl FnOnce(&mut T) -> fmt::Result,
    ) -> fmt::Result {
        let w = &mut self.inner.writer;
        write!(w, "<!{} ", tag)?;
        func(w)?;
        write!(w, ">")?;
        Ok(())
    }

    pub fn prolog<'b, FF>(&'b mut self, tag: &str, func: FF) -> fmt::Result
    where
        for<'x, 'y> FF:
            FnOnce(&'x mut AttrBuilder<'y, T>) -> Result<&'x mut AttrBuilder<'y, T>, fmt::Error>,
    {
        write!(self.inner.writer, "<?{}", tag)?;
        let mut k = AttrBuilder::new(self.inner.writer);
        func(&mut k)?;
        write!(self.inner.writer, "?>")?;
        Ok(())
    }

    pub fn comment(&mut self, func: impl FnOnce(&mut T) -> fmt::Result) -> fmt::Result {
        let w = &mut self.inner.writer;
        write!(w, "<!--")?;
        func(w)?;
        write!(w, " -->")
    }

    pub fn elem_no_attr<'b>(
        &'b mut self,
        tag: &'b str,
    ) -> Result<XML<'b, T, impl FnOnce(&mut T) -> fmt::Result + 'b>, fmt::Error> {
        self.elem(tag, |w| Ok(w))
    }

    /// Before every element is destroyed, the user must manually call
    /// `end()` and handle the error case of writing the end tag failing.
    /// You will get a runtime panic if end() wasnt called on an element.
    ///
    /// If you don't want to worry about remembering to call end(),
    /// You can instead call this function that will automatically all end
    /// After the specified closure happens.
    pub fn defer_end(mut self, func: impl FnOnce(&mut Self) -> fmt::Result) -> fmt::Result {
        func(&mut self)?;
        self.end()
    }

    #[must_use]
    pub fn elem<'b, FF>(
        &'b mut self,
        tag: &'b str,
        func: FF,
    ) -> Result<XML<'b, T, impl FnOnce(&mut T) -> fmt::Result + 'b>, fmt::Error>
    where
        for<'x, 'y> FF:
            FnOnce(&'x mut AttrBuilder<'y, T>) -> Result<&'x mut AttrBuilder<'y, T>, fmt::Error>,
    {
        Ok(XML {
            inner: self.inner.elem(
                move |w| {
                    write!(w, "<{}", tag)?;
                    func(&mut AttrBuilder::new(w))?;
                    write!(w, ">")?;
                    Ok(())
                },
                move |w| write!(w, "</{}>", tag),
            )?,
        })
    }

    pub fn end(self) -> Result<(), fmt::Error> {
        self.inner.end()
    }
}
