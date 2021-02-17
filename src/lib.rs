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
    pub use super::element;
    pub use super::empty_element;
    pub use super::end;
    pub use super::new_element;
    pub use super::new_empty_element;
}

///Contains the structs that the element macros work with internally.
pub mod elem;

///Write the ending tag for an element.
#[macro_export]
macro_rules! end {
    ($dst:expr,$($arg:tt)*) => {
        $dst.end(format_args!($($arg)*))
    }
}

///Create a new element from a writer.
#[macro_export]
macro_rules! new_element {
    ($dst:expr,$($arg:tt)*) => {
        $crate::elem::Element::new($dst,format_args!($($arg)*))
    }
}

///Create a new element from another element.
#[macro_export]
macro_rules! element {
    ($dst:expr,$($arg:tt)*) => {
        $dst.borrow(format_args!($($arg)*))
    }
}

///Create a element with no ending tag from an element.
#[macro_export]
macro_rules! empty_element {
    ($dst:expr,$($arg:tt)*) => {
        $dst.borrow_single(format_args!($($arg)*))
    }
}

///Create a element with no ending tag.
#[macro_export]
macro_rules! new_empty_element {
    ($dst:expr,$($arg:tt)*) => {
        $crate::elem::Single::new($dst,format_args!($($arg)*))
    }
}

use core::fmt;

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
