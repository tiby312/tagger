use super::*;

pub struct Element<'a, T: Write> {
    pub(super) writer: &'a mut T,
    pub(super) tag: &'a str,
    pub(super) level: usize,
}

impl<'a, T: Write> Drop for Element<'a, T> {
    fn drop(&mut self) {
        if !self.tag.is_empty() {
            for _ in 0..self.level - 1 {
                let _ = write!(self.writer, "{}", '\t');
            }
            let _ = write!(self.writer, "</{}>\n", self.tag);
        }
    }
}

impl<'a, T: Write> ElementTrait for Element<'a, T> {
    type W = T;
    fn get_writer(&mut self) -> &mut T {
        &mut self.writer
    }
    fn get_level(&self) -> usize {
        self.level
    }
}

pub struct TagBuilder<'a, T: Write> {
    writer: Option<&'a mut T>,
    tag: &'a str,
    level: usize,
}
impl<'a, T: Write> TagBuilderTrait for TagBuilder<'a, T> {
    type W = T;
    //Gives user access to the writer for more control
    //Before it is returned, a space is added.
    fn get_writer(&mut self) -> &mut T {
        self.writer.as_mut().unwrap()
    }
}
impl<'a, T: Write> TagBuilder<'a, T> {
    pub(super) fn new(writer: &'a mut T, tag: &'a str, level: usize) -> TagBuilder<'a, T> {
        TagBuilder {
            writer: Some(writer),
            tag,
            level,
        }
    }

    pub fn empty(mut self) {
        write!(self.writer.take().unwrap(), "{}", "/>\n").unwrap();
    }

    pub fn empty_no_slash(mut self) {
        write!(self.writer.take().unwrap(), "{}", ">\n").unwrap();
    }

    pub fn end(mut self) -> Element<'a, T> {
        let writer = self.writer.take().unwrap();
        write!(writer, "{}", ">\n").unwrap();
        Element {
            writer,
            tag: self.tag,
            level: self.level,
        }
    }
}
impl<'a, T: Write> Drop for TagBuilder<'a, T> {
    fn drop(&mut self) {
        if let Some(writer) = self.writer.take() {
            let _ = write!(writer, "{}", ">\n");
        }
    }
}
