//! This crate provides primitives to build up a html/xml/svg document programatically.
//! Instead of using a templating engine, write data/markup that 'looks like' rust.
//!
//! ### Why so many closures?
//!
//! Unlike Drop, passing closures allows us to guarantee that some code runs that could fail
//! during nominal execution.
//!

pub mod svg;
use svg::*;

///Convenience macro to reduce code.
///Create a closure that will use write!() with the formatting arguments.
#[macro_export]
macro_rules! wr {
    ($($arg:tt)*) => {
        move |w|write!(w,$($arg)*)
    }
}

///The prelude to import the element manipulation convenience macros.
pub mod prelude {
    pub use super::wr;
    pub use super::AttrTrait;
    pub use core::fmt::Write;
}

use core::fmt;

use fmt::Write;

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



///Common tags to be used in [`Element::single_ext`]
pub mod tag_types {
    /// Equivalent to `<{}/>`
    pub static NORMAL: [&'static str; 2] = ["<", "/>"];
    /// Equivalent to `<!--{}-->>`
    pub static COMMENT: [&'static str; 2] = ["<!--", "-->"];
    /// Equivalent to `<?{}?>`
    pub static PROLOG: [&'static str; 2] = ["<?", "?>"];
    /// Equivalent to `<!{}>`
    pub static DECL: [&'static str; 2] = ["<!", ">"];
}

/// Used by [`Element::elem`]
pub struct ElemBuilder<'a, T>(&'a mut Element<T>);

impl<'a, T: Write> ElemBuilder<'a, T> {
    /// Write out the attributes for an element with an ending tag.
    /// Return a build certificate to prove that this build function was run.
    pub fn build<F>(
        self,
        func: F,
    ) -> Result<(&'a mut Element<T>, Result<BuildCertificate, fmt::Error>), fmt::Error>
    where
        for<'x, 'y> F:
            FnOnce(&'x mut AttrBuilder<'y, T>) -> Result<&'x mut AttrBuilder<'y, T>, fmt::Error>,
    {
        let _res = func(&mut AttrBuilder { inner: self.0 });

        write!(self.0, ">")?;
        Ok((self.0, Ok(BuildCertificate(()))))
    }
}

///Use a trait to simplify lifetimes
pub trait AttrTrait: Write + Sized {
    fn polyline_data<'b, F>(&'b mut self, func: F) -> Result<&'b mut Self, fmt::Error>
    where
        for<'x, 'y> F: FnOnce(
            &'x mut PolyLineBuilder<'y, Self>,
        ) -> Result<&'x mut PolyLineBuilder<'y, Self>, fmt::Error>,
    {
        {
            let mut p = PolyLineBuilder::new(self)?;
            func(&mut p)?;
            p.finish()?;
        }
        Ok(self)
    }

    fn path_data<'b, F>(&'b mut self, func: F) -> Result<&'b mut Self, fmt::Error>
    where
        for<'x, 'y> F: FnOnce(
            &'x mut PathBuilder<'y, Self>,
        ) -> Result<&'x mut PathBuilder<'y, Self>, fmt::Error>,
    {
        {
            let mut p = PathBuilder::new(self)?;
            func(&mut p)?;
            p.finish()?;
        }
        Ok(self)
    }

    fn with_attr(
        &mut self,
        s: &str,
        func: impl FnOnce(&mut Self) -> core::fmt::Result,
    ) -> Result<&mut Self, core::fmt::Error> {
        write!(self, "{}=", s)?;
        write!(self, "\"")?;
        func(self)?;
        write!(self, "\" ")?;
        Ok(self)
    }
    fn attr(
        &mut self,
        s: &str,
        val: impl core::fmt::Display,
    ) -> Result<&mut Self, core::fmt::Error> {
        write!(self, "{}=\"{}\" ", s, val)?;
        Ok(self)
    }
}

pub struct AttrBuilder<'a, T> {
    inner: &'a mut Element<T>,
}

impl<'a, T: fmt::Write> fmt::Write for AttrBuilder<'a, T> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.inner.write_str(s)
    }
}
impl<'a, T: fmt::Write> AttrTrait for AttrBuilder<'a, T> {}

pub struct Element<T> {
    writer: T,
}

impl<T: fmt::Write> fmt::Write for Element<T> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.writer.write_str(s)
    }
}

//Struct indicating that the build function was called.
#[must_use]
pub struct BuildCertificate(());

impl<T: fmt::Write> Element<T> {
    /// Create a new element.
    pub fn new(writer: T) -> Self {
        Element { writer }
    }

    /// Write a element that doesnt have an ending tag. i.e. it can only have attributes.
    /// Some common tag types are in [`tag_types`].
    pub fn single_ext<F>(&mut self, tag: &str, tags: [&str; 2], func: F) -> fmt::Result
    where
        for<'x, 'y> F:
            FnOnce(&'x mut AttrBuilder<'y, T>) -> Result<&'x mut AttrBuilder<'y, T>, fmt::Error>,
    {
        let [start, end] = tags;
        write!(self.writer, "{}{} ", start, tag)?;
        func(&mut AttrBuilder { inner: self })?;
        write!(self.writer, "{}", end)
    }

    /// Shorthand for [`Element::single_ext`] with [`tag_types::NORMAL`]
    pub fn single<F>(&mut self, tag: &str, func: F) -> fmt::Result
    where
        for<'x, 'y> F:
            FnOnce(&'x mut AttrBuilder<'y, T>) -> Result<&'x mut AttrBuilder<'y, T>, fmt::Error>,
    {
        self.single_ext(tag, ["<", "/>"], func)
    }

    /// Shorthand for [`Element::elem`] with the attribute builder functionality omitted.
    pub fn elem_no_attr<F>(&mut self, tag: &str, func: F) -> fmt::Result
    where
        for<'x> F: FnOnce(&'x mut Element<T>) -> fmt::Result,
    {
        write!(self.writer, "<{}>", tag)?;
        func(self)?;
        write!(self.writer, "</{}>", tag)
    }

    /// Write a element that has an ending tag.
    pub fn elem<F>(&mut self, tag: &str, func: F) -> fmt::Result
    where F: FnOnce(ElemBuilder<T>) -> Result<BuildCertificate, fmt::Error>,
    {
        write!(self.writer, "<{} ", tag)?;
        let attr = ElemBuilder(self);

        let _cert = func(attr)?;

        write!(self.writer, "</{}>", tag)
    }
}
