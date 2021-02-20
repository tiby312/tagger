//! This crate provides primitives to build up a html/xml/svg document programatically.
//! Instead of using a templating engine, write data/markup that 'looks like' rust.
//!
//! ### Why so many closures?
//!
//! Closures are great because they can be passed to a function.
//! Unlike Drop, this allows us to guarantee that some code runs that could fail
//! during nominal execution.
//!

pub mod xml;

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
    pub use super::xml::AttrTrait;
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
