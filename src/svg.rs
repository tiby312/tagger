use super::*;    

pub struct PathCommander<'a, 'b> {
    writer: &'a mut fmt::Formatter<'b>,
}

impl<'a, 'b> PathCommander<'a, 'b> {
    pub fn close(&mut self) -> fmt::Result {
        write!(self.writer, "z")
    }
    pub fn move_to(&mut self, point: [f32; 2]) -> Result<&mut Self, fmt::Error> {
        write!(self.writer, "M {} {} ", point[0], point[1])?;
        Ok(self)
    }
    pub fn line_to(&mut self, point: [f32; 2]) -> Result<&mut Self, fmt::Error> {
        write!(self.writer, "L {} {} ", point[0], point[1])?;
        Ok(self)
    }
}

pub fn path(func: impl FnOnce(PathCommander) -> fmt::Result) -> impl fmt::Display {
    struct Path<F: FnOnce(PathCommander) -> fmt::Result> {
        it: std::cell::RefCell<Option<F>>,
    }
    impl<F: FnOnce(PathCommander) -> fmt::Result> fmt::Display for Path<F> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let comm = PathCommander { writer: f };
            (self.it.borrow_mut().take().unwrap())(comm)?;
            Ok(())
        }
    }

    Path {
        it: std::cell::RefCell::new(Some(func)),
    }
}

pub fn poly(a: impl ExactSizeIterator<Item = [f32; 2]>) -> impl fmt::Display {
    struct PolyLine<I> {
        it: std::cell::RefCell<I>,
    }
    impl<I: Iterator<Item = [f32; 2]>> PolyLine<I> {
        fn new(it: I) -> PolyLine<I> {
            PolyLine {
                it: std::cell::RefCell::new(it),
            }
        }
    }
    impl<I: Iterator<Item = [f32; 2]>> fmt::Display for PolyLine<I> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            for [x, y] in &mut *self.it.borrow_mut() {
                write!(f, "{},{} ", x, y)?
            }
            Ok(())
        }
    }
    PolyLine::new(a)
}