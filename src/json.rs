use super::*;
use crate::raw::Element;

pub fn json<'a, T: Write>(
    writer: &'a mut T,
) -> Result<JSON<'a, T, impl FnOnce(&mut T) -> fmt::Result + 'a>, fmt::Error> {
    write!(writer, "{{")?;
    Ok(JSON {
        inner: Element::new(writer, move |w| write!(w, "}}")),
        atleast_one_attr: false,
    })
}
pub struct JSON<'a, T, F> {
    inner: Element<'a, T, F>,
    atleast_one_attr: bool,
}
impl<'a, T: Write, F: FnOnce(&mut T) -> fmt::Result> JSON<'a, T, F> {
    pub fn elem<'b>(
        &'b mut self,
        tag: &'b str,
    ) -> Result<JSON<'b, T, impl FnOnce(&mut T) -> fmt::Result + 'b>, fmt::Error> {
        let atleast_one_attr = self.atleast_one_attr;
        Ok(JSON {
            inner: self.inner.elem(
                move |w| {
                    if atleast_one_attr {
                        write!(w, ",\"{}\":{{", tag)?
                    } else {
                        write!(w, "\"{}\":{{", tag)?
                    }
                    Ok(())
                },
                move |w| write!(w, "}}"),
            )?,
            atleast_one_attr: false,
        })
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

    pub fn inner(&mut self, s: &str, f: impl core::fmt::Display) -> fmt::Result {
        if self.atleast_one_attr {
            write!(self.inner.writer, ",\"{}\":\"{}\"", s, f)?
        } else {
            write!(self.inner.writer, "\"{}\":\"{}\"", s, f)?
        }
        self.atleast_one_attr = true;
        Ok(())
    }

    pub fn end(self) -> fmt::Result {
        self.inner.end()
    }
}
