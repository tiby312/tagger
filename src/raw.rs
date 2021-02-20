use super::*;
///The base element structure.
///It will panic if the user doesnt properly
///call end() on it.
pub struct Element<'a, T, F> {
    pub writer: &'a mut T,
    func: Option<F>,
}
impl<'a, T: Write, F: FnOnce(&mut T) -> fmt::Result> Element<'a, T, F> {
    pub fn get_writer(&mut self) -> &mut T {
        self.writer
    }
    ///Write an element.
    pub fn new(writer: &'a mut T, func: F) -> Element<'a, T, F> {
        Element {
            writer,
            func: Some(func),
        }
    }

    ///Write an element with no end tag.
    pub fn single(&mut self, a: impl FnOnce(&mut T) -> fmt::Result) -> fmt::Result {
        (a)(self.writer)
    }

    ///Start a new element.
    pub fn elem_tuple<'b, F0: FnOnce(&mut T) -> fmt::Result, F1: FnOnce(&mut T) -> fmt::Result>(
        &'b mut self,
        a: (F0, F1),
    ) -> Result<Element<'b, T, F1>, fmt::Error> {
        (a.0)(self.writer)?;
        Ok(Element {
            writer: self.writer,
            func: Some(a.1),
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
            func: Some(func),
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
