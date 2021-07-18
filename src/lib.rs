use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

/// Each function will only be run exactly once!!!!
trait Elem {
    fn header(&self, f: &mut Formatter<'_>) -> fmt::Result;
    fn end(&self, f: &mut Formatter<'_>) -> fmt::Result;
}

struct ElementWrapper<T: Elem, J: Elem> {
    a: T,
    b: J,
}

impl<T: Elem, J: Elem> Elem for ElementWrapper<T, J> {
    fn header(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.a.header(f)?;
        self.b.header(f)?;
        self.b.end(f)
    }

    fn end(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.a.end(f)
    }
}

struct Empty;
impl Elem for Empty {
    fn header(&self, _: &mut Formatter<'_>) -> fmt::Result {
        Ok(())
    }

    fn end(&self, _: &mut Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

pub struct Element<'a> {
    inner: InnerElem<'a>,
}

struct InnerElem<'a> {
    inner: Box<dyn Elem + 'a>,
}

impl<'a> InnerElem<'a> {
    fn new(inner: impl Elem + 'a) -> Self {
        InnerElem {
            inner: Box::new(inner),
        }
    }
}
impl<'a> Elem for InnerElem<'a> {
    fn header(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.inner.header(f)
    }
    fn end(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.inner.end(f)
    }
}

impl<'a> Elem for Element<'a> {
    fn header(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.inner.inner.header(f)
    }

    fn end(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.inner.inner.end(f)
    }
}
impl<'a> Display for Element<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.header(f)?;
        self.end(f)
    }
}
impl<'a> Element<'a> {
    pub fn append(&mut self, b: Element<'a>) {
        let mut a = InnerElem::new(Empty);
        core::mem::swap(&mut a, &mut self.inner);
        let e = ElementWrapper { a, b };

        self.inner = InnerElem { inner: Box::new(e) };
    }
}

pub fn element<'a, A: Display + 'a, B: Display + 'a>(header: A, end: B) -> Element<'a> {
    struct DisplayElement<A, B> {
        header: A,
        end: B,
    }

    impl<A: Display, B: Display> Elem for DisplayElement<A, B> {
        fn header(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.header)
        }

        fn end(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.end)
        }
    }

    Element {
        inner: InnerElem::new(DisplayElement { header, end }),
    }
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
        }
    }
}

pub struct Path<'a> {
    inner: Element<'a>,
}

impl<'a> Display for Path<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

pub struct Attr<'a> {
    inner: Element<'a>,
}

impl<'a> Display for Attr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

pub struct AttrBuilder<'a> {
    inner: Element<'a>,
}
impl<'a> AttrBuilder<'a> {
    pub fn new() -> Self {
        AttrBuilder {
            inner: elem_single!(""),
        }
    }
    pub fn attr_whole(&mut self, a: impl Display + 'a) -> &mut Self {
        self.inner.append(elem_single!(a));
        self
    }
    pub fn attr(&mut self, name: impl Display + 'a, b: impl Display + 'a) -> &mut Self {
        self.inner
            .append(elem_single!(move_format!("{}=\"{}\" ", name, b)));
        self
    }
    pub fn finish(&mut self) -> Attr<'a> {
        let mut k = elem_single!("");
        core::mem::swap(&mut k, &mut self.inner);
        Attr { inner: k }
    }
}

/// Create the attribute for a svg polyline or polygon.
pub struct PathBuilder<'a> {
    inner: Element<'a>,
}
impl<'a> PathBuilder<'a> {
    pub fn new() -> Self {
        PathBuilder {
            inner: elem_single!("d=\""),
        }
    }
    pub fn draw_z(&mut self) -> &mut Self {
        self.inner.append(elem_single!("Z"));
        self
    }
    pub fn draw<F: fmt::Display + 'a>(&mut self, val: PathCommand<F>) -> &mut Self {
        self.inner
            .append(elem_single!(moveable_format(move |f| val.write(f))));
        self
    }

    pub fn finish(&mut self) -> Path<'a> {
        self.inner.append(elem_single!("\""));
        let mut k = empty_elem!("");
        core::mem::swap(&mut k, &mut self.inner);
        Path { inner: k }
    }
}

pub struct Points<'a> {
    inner: Element<'a>,
}

impl<'a> Display for Points<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

/// Create the attribute for a svg polyline or polygon.
pub struct PointsBuilder<'a> {
    inner: Element<'a>,
}
impl<'a> PointsBuilder<'a> {
    pub fn new() -> Self {
        PointsBuilder {
            inner: elem_single!("points=\""),
        }
    }
    pub fn add(&mut self, x: impl fmt::Display + 'a, y: impl fmt::Display + 'a) -> &mut Self {
        self.inner
            .append(elem_single!(move_format!("{},{} ", x, y)));
        self
    }

    pub fn finish(&mut self) -> Points<'a> {
        self.inner.append(elem_single!("\""));
        let mut k = empty_elem!("");
        core::mem::swap(&mut k, &mut self.inner);
        Points { inner: k }
    }
}

/// Shorthand for `moveable_format(move |w|write!(w,...))`
/// Similar to `format_args!()` except has a more flexible lifetime.
#[macro_export]
macro_rules! move_format {
    ($($arg:tt)*) => {
        $crate::moveable_format(move |w| write!(w,$($arg)*))
    }
}

/// Convert a moved closure into a impl fmt::Display.
/// This is useful because std's `format_args!()` macro
/// has a shorter lifetime.
pub fn moveable_format(func: impl Fn(&mut fmt::Formatter) -> fmt::Result) -> impl fmt::Display {
    struct Foo<F>(F);
    impl<F: Fn(&mut fmt::Formatter) -> fmt::Result> fmt::Display for Foo<F> {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            (self.0)(formatter)
        }
    }
    Foo(func)
}

// This is a simple macro named `say_hello`.
#[macro_export]
macro_rules! elem_single {
    // `()` indicates that the macro takes no argument.
    ($a:expr) => {
        // The macro will expand into the contents of this block.
        element($a, "");
    };
}

