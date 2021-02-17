use core::fmt;
use core::fmt::Write;

/// A non-lifetimed `Element`.
///
/// The [`Element`] struct requires a lifetime.
/// If you want to pass an element between functions, this can be difficult.
/// Instead you can use this struct.
///
/// Unlike `Element`, the user specifies the end tag upfront. This is done
/// To make it easier for a caller who might not have knowledge of what
/// elements are on the stack.
///
/// If you make a function that returns this struct, the user should call
/// `finish()` because they probably do not know how many elements are on the stack.
///
pub struct ElementStack<T: Write> {
    writer: T,
    ends: Vec<String>,
}

impl<T: Write> ElementStack<T> {
    pub fn new(mut writer: T, ar: fmt::Arguments, last: impl ToString) -> Result<Self, fmt::Error> {
        writer.write_fmt(ar)?;

        Ok(ElementStack {
            writer,
            ends: vec![last.to_string()],
        })
    }

    ///Unwind all end tags on the stack.
    pub fn finish(mut self) -> fmt::Result {
        for s in self.ends.iter().rev() {
            write!(self.writer, "{}", s)?;
        }
        self.ends.clear();
        Ok(())
    }
    pub fn end_last(&mut self) -> fmt::Result {
        let s = self.ends.pop().unwrap();
        write!(self.writer, "{}", s)?;
        Ok(())
    }

    pub fn write_str(&mut self, s: &str) -> fmt::Result {
        write!(self.writer, "{}", s)
    }
    pub fn get_writer(&mut self) -> &mut T {
        &mut self.writer
    }

    pub fn borrow_move(
        &mut self,
        a: fmt::Arguments,
        last: impl ToString,
    ) -> Result<(), fmt::Error> {
        self.writer.write_fmt(a)?;
        self.ends.push(last.to_string());
        Ok(())
    }

    //We can't use DerefMut because we this struct is not lifetimed.
    pub fn borrow<'b>(&'b mut self) -> Single<'b, T> {
        Single {
            writer: &mut self.writer,
        }
    }
}

impl<T: Write> Drop for ElementStack<T> {
    fn drop(&mut self) {
        //Runtime checked linear types.
        //we do this to force the user to handle the result of
        //writing the last tag failing.

        if !self.ends.is_empty() && !std::thread::panicking() {
            //TODO print out element
            panic!("end() was not called on these elements {:?}", self.ends);
        }
    }
}

///
/// An element that does not have an end tag and thus
/// does not require that any code be run after it is created.
///
#[repr(transparent)]
pub struct Single<'a, T: Write> {
    writer: &'a mut T,
}
impl<'a, T: Write> Single<'a, T> {
    pub fn new(writer: &'a mut T, ar: fmt::Arguments) -> Result<Self, fmt::Error> {
        writer.write_fmt(ar)?;
        Ok(Single { writer })
    }

    pub fn borrow<'b>(&'b mut self, a: fmt::Arguments<'_>) -> Result<Element<'b, T>, fmt::Error> {
        let w = &mut self.writer;
        w.write_fmt(a)?;
        Ok(Element { writer: Some(w) })
    }
    pub fn borrow_single<'b>(&'b mut self, a: fmt::Arguments) -> Result<Single<'b, T>, fmt::Error> {
        let w = &mut self.writer;
        w.write_fmt(a)?;
        Ok(Single { writer: w })
    }
}

///
/// An element with a ending tag.
/// Once, constructed, the user must call `end()`,
/// in order to write and handle the error case of writing the
/// end tag.
///
pub struct Element<'a, T: Write> {
    writer: Option<&'a mut T>,
}
impl<'a, T: Write> Drop for Element<'a, T> {
    fn drop(&mut self) {
        //Runtime checked linear types.
        //we do this to force the user to handle the result of
        //writing the last tag failing.
        if !self.writer.is_none() && !std::thread::panicking() {
            panic!("end() must be called on this element");
        }
    }
}

impl<'a, T: Write> core::ops::Deref for Element<'a, T> {
    type Target = Single<'a, T>;
    fn deref(&self) -> &Single<'a, T> {
        let m: &T = &*self.writer.as_ref().unwrap();
        unsafe { &*(m as *const _ as *const _) }
    }
}
impl<'a, T: Write> core::ops::DerefMut for Element<'a, T> {
    fn deref_mut(&mut self) -> &mut Single<'a, T> {
        let m: &mut T = &mut *self.writer.as_mut().unwrap();
        unsafe { &mut *(m as *mut _ as *mut _) }
    }
}

impl<'a, T: Write> Element<'a, T> {
    pub fn end(mut self, a: fmt::Arguments) -> fmt::Result {
        self.writer.take().unwrap().write_fmt(a)
    }
    pub fn new(writer: &'a mut T, ar: fmt::Arguments) -> Result<Self, fmt::Error> {
        writer.write_fmt(ar)?;
        Ok(Element {
            writer: Some(writer),
        })
    }
    pub fn write_str(&mut self, s: &str) -> fmt::Result {
        write!(self.writer.as_mut().unwrap(), "{}", s)
    }
    pub fn get_writer(&mut self) -> &mut T {
        self.writer.as_mut().unwrap()
    }
    pub fn borrow<'b>(&'b mut self, a: fmt::Arguments) -> Result<Element<'b, T>, fmt::Error> {
        let w = self.writer.as_mut().unwrap();
        w.write_fmt(a)?;
        Ok(Element { writer: Some(w) })
    }
    pub fn borrow_single<'b>(&'b mut self, a: fmt::Arguments) -> Result<Single<'b, T>, fmt::Error> {
        let w = self.writer.as_mut().unwrap();
        w.write_fmt(a)?;
        Ok(Single { writer: w })
    }
}
