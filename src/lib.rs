use std::fmt;

#[cfg(doctest)]
mod test_readme {
    macro_rules! external_doc_test {
        ($x:expr) => {
            #[doc = $x]
            extern "C" {}
        };
    }

    external_doc_test!(include_str!("../README.md"));
}

///
/// Construct and Write a SVG path's data.
///
/// following: [w3 spec](https://www.w3.org/TR/SVG/paths.html#PathDataGeneralInformation)
///
pub enum PathCommand<F: fmt::Display> {
    /// move to
    M(F, F),
    /// relative move to
    M_(F, F),
    /// line to
    L(F, F),
    /// relative line to
    L_(F, F),
    /// horizontal to
    H(F),
    /// relative horizontal to
    H_(F),
    /// vertical to
    V(F),
    /// relative vertical to
    V_(F),
    /// curve to
    C(F, F, F, F, F, F),
    /// relative curve to
    C_(F, F, F, F, F, F),
    /// shorthand curve to
    S(F, F, F, F),
    /// relative shorthand curve to
    S_(F, F, F, F),
    /// quadratic bezier curve to
    Q(F, F, F, F),
    /// relative quadratic bezier curve to
    Q_(F, F, F, F),
    /// shorthand quadratic bezier curve to
    T(F, F),
    /// relative shorthand quadratic bezier curve to
    T_(F, F),
    /// elliptical arc
    A(F, F, F, F, F, F, F),
    /// relative elliptical arc
    A_(F, F, F, F, F, F, F),
    /// close path
    Z(F),
}

impl<F: fmt::Display> PathCommand<F> {
    fn write<T: fmt::Write>(&self, writer: &mut T) -> fmt::Result {
        use PathCommand::*;
        match self {
            M(x, y) => {
                write!(writer, " M {} {}", x, y)
            }
            M_(x, y) => {
                write!(writer, " m {} {}", x, y)
            }
            L(x, y) => {
                write!(writer, " L {} {}", x, y)
            }
            L_(x, y) => {
                write!(writer, " l {} {}", x, y)
            }
            H(a) => {
                write!(writer, " H {}", a)
            }
            H_(a) => {
                write!(writer, " h {}", a)
            }
            V(a) => {
                write!(writer, " V {}", a)
            }
            V_(a) => {
                write!(writer, " v {}", a)
            }
            C(x1, y1, x2, y2, x, y) => {
                write!(writer, " C {} {}, {} {}, {} {}", x1, y1, x2, y2, x, y)
            }
            C_(dx1, dy1, dx2, dy2, dx, dy) => {
                write!(writer, " c {} {}, {} {}, {} {}", dx1, dy1, dx2, dy2, dx, dy)
            }
            S(x2, y2, x, y) => {
                write!(writer, " S {},{} {} {}", x2, y2, x, y)
            }
            S_(x2, y2, x, y) => {
                write!(writer, " s {},{} {} {}", x2, y2, x, y)
            }
            Q(x1, y1, x, y) => {
                write!(writer, " Q {} {}, {} {}", x1, y1, x, y)
            }
            Q_(dx1, dy1, dx, dy) => {
                write!(writer, " q {} {}, {} {}", dx1, dy1, dx, dy)
            }
            T(x, y) => {
                write!(writer, " T {} {}", x, y)
            }
            T_(x, y) => {
                write!(writer, " t {} {}", x, y)
            }
            A(rx, ry, x_axis_rotation, large_arc_flag, sweep_flag, x, y) => {
                write!(
                    writer,
                    " A {} {} {} {} {} {} {}",
                    rx, ry, x_axis_rotation, large_arc_flag, sweep_flag, x, y
                )
            }
            A_(rx, ry, x_axis_rotation, large_arc_flag, sweep_flag, dx, dy) => {
                write!(
                    writer,
                    " a {} {} {} {} {} {} {}",
                    rx, ry, x_axis_rotation, large_arc_flag, sweep_flag, dx, dy
                )
            }
            Z(_) => {
                write!(writer, " Z")
            }
        }
    }
}

///
/// Build a path.
///
pub struct PathBuilder<'a, T> {
    writer: &'a mut T,
}
impl<'a, T: fmt::Write> PathBuilder<'a, T> {
    pub fn add(&mut self, command: crate::PathCommand<impl fmt::Display>) -> &mut Self {
        command.write(&mut self.writer).unwrap();
        self
    }
}

///
/// Build up a list of points.
///
pub struct PointsBuilder<'a, T> {
    writer: &'a mut T,
}
impl<'a, T: fmt::Write> PointsBuilder<'a, T> {
    pub fn add(&mut self, x: impl fmt::Display, y: impl fmt::Display) -> &mut Self {
        write!(self.writer, "{},{} ", x, y).unwrap();
        self
    }
}

///
/// Used to wrap a `std::io::Write` to have `std::fmt::Write`.
/// The underlying error can be extracted through the error field.
///
pub struct Adaptor<T> {
    pub inner: T,
    pub error: Result<(), std::io::Error>,
}

///
/// Create an initial `ElemWriter`
///
pub fn new<T: fmt::Write>(a: T) -> ElemWriter<T> {
    ElemWriter(a)
}

///
/// Create an `ElemWriter` from a `std::io::Write`
///
pub fn from_io<T: std::io::Write>(a: T) -> ElemWriter<Adaptor<T>> {
    ElemWriter(upgrade_write(a))
}

///Update a `std::io::Write` to be a `std::fmt::Write`
fn upgrade_write<T: std::io::Write>(inner: T) -> Adaptor<T> {
    Adaptor {
        inner,
        error: Ok(()),
    }
}

impl<T: std::io::Write> std::fmt::Write for Adaptor<T> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        match self.inner.write_all(s.as_bytes()) {
            Ok(()) => Ok(()),
            Err(e) => {
                self.error = Err(e);
                Err(std::fmt::Error)
            }
        }
    }
}

///
/// A struct that captures a half-made element. To
/// complete building an element, `build()` must be called.
///
#[must_use]
pub struct ElementBridge<'a, T: fmt::Write, D: fmt::Display> {
    writer: &'a mut ElemWriter<T>,
    tag: D,
}
impl<'a, T: fmt::Write, D: fmt::Display> ElementBridge<'a, T, D> {
    pub fn build(self, func: impl FnOnce(&mut ElemWriter<T>)) -> &'a mut ElemWriter<T> {
        func(self.writer);
        write!(self.writer.0, "</{}>", self.tag).unwrap();
        self.writer
    }
}

///
/// Create attributes.
///
pub struct AttrWriter<'a, T: fmt::Write>(&'a mut T);
impl<'a, T: fmt::Write> AttrWriter<'a, T> {
    pub fn attr(&mut self, a: impl fmt::Display, b: impl fmt::Display) -> &mut Self {
        write!(self.0, " {}=\"{}\"", a, b).unwrap();
        self
    }
    pub fn writer(&mut self) -> &mut T {
        &mut self.0
    }
    pub fn add_raw(&mut self, a: impl fmt::Display) -> &mut Self {
        write!(self.0, " {}", a).unwrap();
        self
    }
    pub fn path(&mut self, a: impl FnOnce(&mut PathBuilder<T>)) -> &mut Self {
        let mut p = PathBuilder { writer: self.0 };
        write!(p.writer, " d=\"").unwrap();
        a(&mut p);
        write!(p.writer, "\"").unwrap();
        self
    }
    pub fn points(&mut self, a: impl FnOnce(&mut PointsBuilder<T>)) -> &mut Self {
        let mut p = PointsBuilder { writer: self.0 };
        write!(p.writer, " points=\"").unwrap();
        a(&mut p);
        write!(p.writer, "\"").unwrap();
        self
    }
}

///
/// Create elements with a start and end tag, or elements with a single tag.
///
pub struct ElemWriter<T: fmt::Write>(T);

impl<T: fmt::Write> ElemWriter<T> {
    pub fn writer(&mut self) -> &mut T {
        &mut self.0
    }
    pub fn add_raw(&mut self, a: impl fmt::Display) -> &mut Self {
        write!(self.0, " {}", a).unwrap();
        self
    }

    pub fn single<D: fmt::Display>(
        &mut self,
        tag: D,
        func: impl FnOnce(&mut AttrWriter<T>),
    ) -> &mut Self {
        write!(self.0, "<{} ", tag).unwrap();
        func(&mut AttrWriter(&mut self.0));
        write!(self.0, " />").unwrap();
        self
    }
    pub fn elem<D: fmt::Display>(
        &mut self,
        tag: D,
        func: impl FnOnce(&mut AttrWriter<T>),
    ) -> ElementBridge<T, D> {
        write!(self.0, "<{} ", tag).unwrap();

        func(&mut AttrWriter(&mut self.0));

        write!(self.0, " >").unwrap();

        ElementBridge { writer: self, tag }
    }
}
