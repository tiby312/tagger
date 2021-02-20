//! This crate provides primitives to build up a html/xml/svg document programatically.
//! Instead of using a templating engine, write data/markup that 'looks like' rust.
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

pub mod json;
pub mod raw;
pub mod xml;
pub mod xml2;

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
        move |w| write!(w, "{}", $arg)
    };
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
