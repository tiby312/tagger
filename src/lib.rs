//! This crate provides primitives to build up a html/xml/svg document programatically.
//! Instead of using a templating engine, write data/markup that 'looks like' rust.
//!
//! ### Why so many closures?
//!
//! Unlike Drop, passing closures can be used to force the user to handle errors when
//! something goes out of scope. If we put the writing of end tags in a Drop method
//! it could silently fail, which is not ideal. This can be worked around by adding
//! an explicit function to write the end tag, but there is no way to guarantee
//! that this function gets called at compile time. The best you can do is a runtime
//! panic if the finalizer function isn't called in order to handle the error case.
//! With closures, you can force a compile-time error.
//!
//! ### Examples
//!
//! See it in use in the [poloto crate](https://crates.io/crates/poloto).
//! Also check out the examples at [github](https://github.com/tiby312/tagger/tree/master/examples).
//!
//!

pub mod svg;
use svg::*;

/// Convenience macro to reduce code.
/// Shorthand for 'move |w|write!(w,...)`
/// Create a closure that will use write!() with the formatting arguments.
#[macro_export]
macro_rules! wr {
    ($($arg:tt)*) => {
        move |w|write!(w,$($arg)*)
    }
}

/// [`fmt::Write::write_fmt`] doesn't return itself on success. This version does.
pub fn write_fmt_ret<'a, T: fmt::Write>(
    w: &'a mut T,
    args: fmt::Arguments,
) -> Result<&'a mut T, fmt::Error> {
    w.write_fmt(args)?;
    Ok(w)
}

/// Just like the regular [`write!()`] macro except it returns itself upon success.
#[macro_export]
macro_rules! write_ret {
    ($dst:expr, $($arg:tt)*) => ($crate::write_fmt_ret($dst,format_args!($($arg)*)))
}

///The prelude to import the element manipulation convenience macros.
pub mod prelude {
    pub use super::wr;
    pub use super::write_ret;
    pub use super::WriteAttr;
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
    pub static NORMAL: [&str; 2] = ["<", "/>"];
    /// Equivalent to `<!--{}-->`
    pub static COMMENT: [&str; 2] = ["<!--", "-->"];
    /// Equivalent to `<?{}?>`
    pub static PROLOG: [&str; 2] = ["<?", "?>"];
    /// Equivalent to `<!{}>`
    pub static DECL: [&str; 2] = ["<!", ">"];
}

/// Used by [`Element::elem`]
pub struct ElementHeaderWriter<'a, T>(&'a mut Element<T>);

impl<'a, T: Write> ElementHeaderWriter<'a, T> {
    /// Write out the attributes for an element with an ending tag.
    pub fn write<F>(self, func: F) -> Result<&'a mut Element<T>, fmt::Error>
    where
        for<'x, 'y> F: FnOnce(
            &'x mut AttributeWriter<'y, T>,
        ) -> Result<&'x mut AttributeWriter<'y, T>, fmt::Error>,
    {
        let _res = func(&mut AttributeWriter { inner: self.0 });

        write!(self.0, ">")?;
        Ok(self.0)
    }
}

/// Functions the user can call to add attributes.
/// [`AttributeWriter`] could have implemented these, but lets use a trait to simplify lifetimes.
pub trait WriteAttr: Write + Sized {
    ///Write the data attribute for a svg polyline or polygon.
    fn points_data<F>(&mut self, func: F) -> Result<&mut Self, fmt::Error>
    where
        for<'x, 'y> F: FnOnce(
            &'x mut PointsBuilder<'y, Self>,
        ) -> Result<&'x mut PointsBuilder<'y, Self>, fmt::Error>,
    {
        {
            let mut p = PointsBuilder::new(self)?;
            func(&mut p)?;
            p.finish()?;
        }
        Ok(self)
    }

    ///Write the data attribute for a svg path.
    fn path_data<F>(&mut self, func: F) -> Result<&mut Self, fmt::Error>
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

    ///Write an attribute where the user can write the value part using [`wr`] macro or the [`write`] macro
    fn with_attr(
        &mut self,
        s: &str,
        func: impl FnOnce(&mut Self) -> core::fmt::Result,
    ) -> Result<&mut Self, core::fmt::Error> {
        write!(self, " {}=", s)?;
        write!(self, "\"")?;
        func(self)?;
        write!(self, "\"")?;
        Ok(self)
    }

    ///Write an attribute with the specified tag and value using the values [`fmt::Display`] trait.
    fn attr(
        &mut self,
        s: &str,
        val: impl core::fmt::Display,
    ) -> Result<&mut Self, core::fmt::Error> {
        write!(self, " {}=\"{}\"", s, val)?;
        Ok(self)
    }
}

///Builder to write out attributes to an element.
pub struct AttributeWriter<'a, T> {
    inner: &'a mut Element<T>,
}

impl<'a, T: fmt::Write> fmt::Write for AttributeWriter<'a, T> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.inner.write_str(s)
    }
}
impl<'a, T: fmt::Write> WriteAttr for AttributeWriter<'a, T> {}

///Sometimes, having to deal with all the borrowing and closures is difficult. For these cases
///a traditional ElementStack can be used. This allows you to move around the stack between functions and
///classes easily. The downside is that you lose compile time assurance that every push matches every pop.
pub struct ElementStack<'a, T> {
    writer: Element<T>,
    tags: Vec<&'a str>,
}
impl<'a, T: fmt::Write> ElementStack<'a, T> {
    pub fn check_unwound(&mut self) {
        if !self.tags.is_empty() {
            panic!("not all ending tags have been popped.")
        }
    }
    pub fn new(writer: T) -> ElementStack<'a, T> {
        ElementStack {
            writer: Element::new(writer),
            tags: Vec::new(),
        }
    }
    pub fn from_element(writer: Element<T>) -> ElementStack<'a, T> {
        ElementStack {
            writer,
            tags: Vec::new(),
        }
    }
    /// Write a element that has an ending tag.
    /// The user is required to feed the element back into this function
    /// thus proving that they called [`ElementHeaderWriter::write`].
    pub fn elem_stack<F>(&mut self, tag: &'a str, func: F) -> Result<&mut Self, fmt::Error>
    where
        for<'x, 'y> F: FnOnce(
            &'x mut AttributeWriter<'y, T>,
        ) -> Result<&'x mut AttributeWriter<'y, T>, fmt::Error>,
    {
        write!(self.writer, "<{}", tag)?;
        let mut attr = AttributeWriter {
            inner: &mut self.writer,
        };

        let _cert = func(&mut attr)?;

        write!(self.writer, ">")?;

        self.tags.push(tag);
        //write!(self.writer, "</{}>", tag)?;
        Ok(self)
    }

    ///Use Deref/DerefMut is possible
    pub fn as_element(&mut self) -> &mut Element<T> {
        &mut self.writer
    }
    pub fn pop(&mut self) -> Result<&mut ElementStack<'a, T>, fmt::Error> {
        let tag = self.tags.pop().expect("pop called too many times");
        write!(self.writer, "</{}>", tag)?;
        Ok(self)
    }
}

impl<'a, T> core::ops::Deref for ElementStack<'a, T> {
    type Target = Element<T>;
    fn deref(&self) -> &Self::Target {
        &self.writer
    }
}
impl<'a, T> core::ops::DerefMut for ElementStack<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.writer
    }
}

///An element.
pub struct Element<T> {
    writer: T,
}

impl<T: fmt::Write> fmt::Write for Element<T> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.writer.write_str(s)
    }
}

impl<T: fmt::Write> Element<T> {
    /// Create a new element.
    pub fn new(writer: T) -> Self {
        Element { writer }
    }

    pub fn into_writer(self) -> T {
        self.writer
    }
    pub fn get_writer(&mut self) -> &mut T {
        &mut self.writer
    }

    /// Write a element that doesnt have an ending tag. i.e. it can only have attributes.
    /// Some common tag types are in [`tag_types`].
    pub fn single_ext<F>(
        &mut self,
        tag: &str,
        tags: [&str; 2],
        func: F,
    ) -> Result<&mut Self, fmt::Error>
    where
        for<'x, 'y> F: FnOnce(
            &'x mut AttributeWriter<'y, T>,
        ) -> Result<&'x mut AttributeWriter<'y, T>, fmt::Error>,
    {
        let [start, end] = tags;
        write!(self.writer, "{}{}", start, tag)?;
        func(&mut AttributeWriter { inner: self })?;
        write!(self.writer, "{}", end)?;
        Ok(self)
    }

    /// Shorthand for [`Element::single_ext`] with [`tag_types::NORMAL`]
    pub fn single<F>(&mut self, tag: &str, func: F) -> Result<&mut Self, fmt::Error>
    where
        for<'x, 'y> F: FnOnce(
            &'x mut AttributeWriter<'y, T>,
        ) -> Result<&'x mut AttributeWriter<'y, T>, fmt::Error>,
    {
        self.single_ext(tag, ["<", "/>"], func)?;
        Ok(self)
    }

    /// Shorthand for [`Element::elem`] with the attribute builder functionality omitted.
    pub fn elem_no_attr<F>(&mut self, tag: &str, func: F) -> Result<&mut Self, fmt::Error>
    where
        for<'x> F: FnOnce(&'x mut Element<T>) -> Result<&'x mut Element<T>, fmt::Error>,
    {
        write!(self.writer, "<{}>", tag)?;
        let _ = func(self)?;
        write!(self.writer, "</{}>", tag)?;
        Ok(self)
    }

    /// Write a element that has an ending tag.
    /// The user is required to feed the element back into this function
    /// thus proving that they called [`ElementHeaderWriter::write`].
    pub fn elem<F>(&mut self, tag: &str, func: F) -> Result<&mut Self, fmt::Error>
    where
        F: FnOnce(ElementHeaderWriter<T>) -> Result<&mut Element<T>, fmt::Error>,
    {
        write!(self.writer, "<{}", tag)?;
        let attr = ElementHeaderWriter(self);

        let _cert = func(attr)?;

        write!(self.writer, "</{}>", tag)?;
        Ok(self)
    }
}
