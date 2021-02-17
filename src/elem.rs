use core::fmt;
use core::fmt::Write;

pub struct Single<'a, T: Write> {
    writer: &'a mut T,
}
impl<'a, T: Write> Single<'a, T> {
    pub fn new(writer: &'a mut T, ar: fmt::Arguments) -> Result<Self, fmt::Error> {
        writer.write_fmt(ar)?;
        Ok(Single { writer })
    }

    pub fn borrow<'b>(&'b mut self, a: fmt::Arguments<'_>) -> Result<Element<'b, T>, fmt::Error> {
        self.writer.write_fmt(a)?;
        Ok(Element {
            writer: self.writer,
            func: Some(()),
        })
    }
}
pub struct Element<'a, T: Write> {
    writer: &'a mut T,
    func: Option<()>,
}
impl<'a, T: Write> Drop for Element<'a, T> {
    fn drop(&mut self) {
        //Runtime checked linear types.
        //we do this to force the user to handle the result of
        //writing the last tag failing.
        if !self.func.is_none() && !std::thread::panicking() {
            panic!("should didnt finish");
        }
    }
}
impl<'a, T: Write> Element<'a, T> {
    pub fn end(mut self, a: fmt::Arguments) -> fmt::Result {
        let _ = self.func.take();
        self.writer.write_fmt(a)
    }
    pub fn new(writer: &'a mut T, ar: fmt::Arguments) -> Result<Self, fmt::Error> {
        writer.write_fmt(ar)?;
        Ok(Element {
            writer,
            func: Some(()),
        })
    }
    pub fn write_str(&mut self, s: &str) -> fmt::Result {
        write!(self.writer, "{}", s)
    }
    pub fn get_writer(&mut self) -> &mut T {
        &mut self.writer
    }
    pub fn borrow<'b>(&'b mut self, a: fmt::Arguments) -> Result<Element<'b, T>, fmt::Error> {
        self.writer.write_fmt(a)?;
        Ok(Element {
            writer: self.writer,
            func: Some(()),
        })
    }
    pub fn borrow_single<'b>(&'b mut self, a: fmt::Arguments) -> Result<Single<'b, T>, fmt::Error> {
        self.writer.write_fmt(a)?;
        Ok(Single {
            writer: self.writer,
        })
    }
}
