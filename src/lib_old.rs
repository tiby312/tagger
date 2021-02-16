use core::fmt::Error;
use core::fmt::Write;

///Build elements by borrowing
pub mod element_borrow;
///Build elements by moving
pub mod element_move;

///Include this to use common element and tab building functions.
pub mod prelude {
    pub use super::ElementTrait;
    pub use super::TagBuilderTrait;
}

///Common functionality between [`element_borrow::Element`] and [`element_move::FlatElement`]
pub trait ElementTrait {
    type W: Write;

    fn get_level(&self) -> usize;

    fn get_writer(&mut self) -> &mut Self::W;

    fn write_str(&mut self, s: &str) {
        write!(self.get_writer(), "{}", s).unwrap();
    }

    fn declaration(&mut self, data: &str) {
        for _ in 0..self.get_level() {
            write!(self.get_writer(), "{}", '\t').unwrap();
        }
        write!(self.get_writer(), "<!{}>\n", data).unwrap();
    }
    fn comment(&mut self, data: &str) {
        for _ in 0..self.get_level() {
            write!(self.get_writer(), "{}", '\t').unwrap();
        }
        write!(self.get_writer(), "<-- {} -->\n", data).unwrap();
    }

    fn tag_build<'b>(&'b mut self, tag: &'b str) -> element_borrow::TagBuilder<'b, Self::W> {
        let levels = self.get_level();
        let w = self.get_writer();
        for _ in 0..levels {
            write!(w, "{}", '\t').unwrap();
        }
        write!(w, "<{}", tag).unwrap();
        element_borrow::TagBuilder::new(w, tag, levels + 1)
    }
}

///Common functionality between [`element_borrow::TagBuilder`] and [`element_move::TagBuilderFlat`]
pub trait TagBuilderTrait: Sized {
    type W: Write;
    fn get_writer(&mut self) -> &mut Self::W;

    fn path_data(&mut self) -> PathData<Self::W> {
        PathData::new(self.get_writer())
    }

    fn polyline_data(&mut self) -> PolyLine<Self::W> {
        PolyLine::new(self.get_writer())
    }

    fn append(mut self, s: &str) -> Self {
        let w = self.get_writer();
        write!(w, " {}", s).unwrap();
        self
    }

    fn setw(mut self, attr: &str, func: impl FnOnce(&mut Self::W) -> Result<(), Error>) -> Self {
        let w = self.get_writer();
        write!(w, " {} = \"", attr).unwrap();
        let _ = func(w);
        write!(w, "\"").unwrap();
        self
    }
    fn set<F: core::fmt::Display>(mut self, attr: &str, val: F) -> Self {
        let w = self.get_writer();
        write!(w, " {} = \"{}\"", attr, val).unwrap();
        self
    }
}

///Starting point.
///Doesnt actually write anything out.
pub fn root<T: Write>(writer: T) -> element_move::FlatElement<T> {
    element_move::FlatElement::new(writer)
}

///Created by [`TagBuilderTrait::polyline_data`]
pub struct PolyLine<'a, T: Write> {
    writer: &'a mut T,
}
impl<'a, T: Write> Drop for PolyLine<'a, T> {
    fn drop(&mut self) {
        let _ = write!(self.writer, "\"");
    }
}
impl<'a, T: Write> PolyLine<'a, T> {
    fn new(writer: &'a mut T) -> PolyLine<'a, T> {
        write!(writer, " points=\"").unwrap();
        PolyLine { writer }
    }
    pub fn point(&mut self, point: [f32; 2]) {
        write!(self.writer, "{},{} ", point[0], point[1]).unwrap();
    }
}

///Created by [`TagBuilderTrait::path_data`]
pub struct PathData<'a, T: Write> {
    writer: &'a mut T,
}
impl<'a, T: Write> Drop for PathData<'a, T> {
    fn drop(&mut self) {
        let _ = write!(self.writer, "\"").unwrap();
    }
}
impl<'a, T: Write> PathData<'a, T> {
    fn new(writer: &'a mut T) -> Self {
        write!(writer, " d=\"").unwrap();
        PathData { writer }
    }
    pub fn close(&mut self) {
        write!(self.writer, "z").unwrap();
    }
    pub fn move_to(&mut self, point: [f32; 2]) -> &mut Self {
        write!(self.writer, "M {} {} ", point[0], point[1]).unwrap();
        self
    }
    pub fn line_to(&mut self, point: [f32; 2]) -> &mut Self {
        write!(self.writer, "L {} {} ", point[0], point[1]).unwrap();
        self
    }
}

use std::*;

///Used by [`upgrade_writer`]
pub struct WriterAdaptor<T> {
    pub inner: T,
    pub error: Result<(), io::Error>,
}

///Upgrade a std::io::Write to be a std::fmt::Write
pub fn upgrade_writer<T: std::io::Write>(inner: T) -> WriterAdaptor<T> {
    WriterAdaptor {
        inner,
        error: Ok(()),
    }
}
impl<T: io::Write> fmt::Write for WriterAdaptor<T> {
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