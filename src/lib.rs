//
// # Why do I have to call end!()?
//
// I wanted to force the user to handle the error case
// of writing the end tag failing. If we did this in the destructor of 
// an element, then the write could silently fail.
//
// So we force the user to call end!() by checking
// that it was called in the destructor.
//
// # Why is it a macro?
//
// So that the user can pass format arguments if desired.
//

pub mod prelude {}
pub use inner::Element;
pub use inner::Single;
mod inner {
    use core::fmt;
    use core::fmt::Write;

    pub struct Single<'a, T: Write> {
        writer: &'a mut T,
    }
    impl<'a, T: Write> Single<'a, T> {
        pub fn new(
            writer: &'a mut T,
            ar: fmt::Arguments
        ) -> Result<Self, fmt::Error> {
            writer.write_fmt(ar)?;
            Ok(Single {
                writer,
            })
        }

        pub fn borrow<'b>(
            &'b mut self,
            a: fmt::Arguments<'_>
        ) -> Result<Element<'b, T>, fmt::Error> {
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
            if !self.func.is_none() && !std::thread::panicking(){
                panic!("should didnt finish");
            }
        }
    }
    impl<'a, T: Write> Element<'a, T> {
        
        pub fn end(mut self,a:fmt::Arguments)->fmt::Result{
            let _ = self.func.take();
            self.writer.write_fmt(a)
        }
        pub fn new(
            writer: &'a mut T,
            ar: fmt::Arguments
        ) -> Result<Self, fmt::Error> {
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
        pub fn borrow<'b>(
            &'b mut self,
            a: fmt::Arguments
        ) -> Result<Element<'b, T>, fmt::Error> {
            self.writer.write_fmt(a)?;
            Ok(Element {
                writer: self.writer,
                func: Some(()),
            })
        }
        pub fn borrow_single<'b>(
            &'b mut self,
            a:fmt::Arguments
        ) -> Result<Single<'b, T>, fmt::Error> {
            self.writer.write_fmt(a)?;
            Ok(Single {
                writer: self.writer,
            })
        }
    }
}

#[macro_export]
macro_rules! end {
    ($dst:expr,$($arg:tt)*) => {
        $dst.end(format_args!($($arg)*))
    }
}


#[macro_export]
macro_rules! new_element {
    ($dst:expr,$($arg:tt)*) => {
        $crate::Element::new($dst,format_args!($($arg)*))
    }
}
#[macro_export]
macro_rules! element {
    ($dst:expr,$($arg:tt)*) => {
        $dst.borrow(format_args!($($arg)*))
    }
}

#[macro_export]
macro_rules! empty_element {
    ($dst:expr,$($arg:tt)*) => {
        $dst.borrow_single(format_args!($($arg)*))
    }
}


#[macro_export]
macro_rules! new_empty_element {
    ($dst:expr,$($arg:tt)*) => {
        $crate::Single::new($dst,format_args!($($arg)*))
    }
}

use core::fmt;

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

///Used by [`upgrade_writer`]
pub struct WriterAdaptor<T> {
    pub inner: T,
    pub error: Result<(), std::io::Error>,
}

///Upgrade a std::io::Write to be a std::fmt::Write
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
