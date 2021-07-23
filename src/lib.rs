use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

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

/// The tagger prelude
pub mod prelude {
    pub use crate::elem;
    pub use crate::formatm;
    pub use crate::single;
}

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

/// The main building block
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
    /// Create an element.
    pub fn new<A: Display + 'a, B: Display + 'a>(header: A, end: B) -> Element<'a> {
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

    /// Move equivalent of `append`
    pub fn add(mut self, b: Element<'a>) -> Self {
        self.append(b);
        self
    }

    /// Append an element. The passed element will be inserted between
    /// the first and second sections of the current element.
    pub fn append(&mut self, b: Element<'a>) -> &mut Self {
        let mut a = InnerElem::new(Empty);
        core::mem::swap(&mut a, &mut self.inner);
        let e = ElementWrapper { a, b };

        self.inner = InnerElem { inner: Box::new(e) };
        self
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

/// A finished path
pub struct Path<'a> {
    inner: Element<'a>,
}

impl<'a> Display for Path<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

/// A finished list of attributes.
pub struct Attr<'a> {
    inner: Element<'a>,
}

impl<'a> Display for Attr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

/// Builder to help make a list of attributes.
pub struct AttrBuilder<'a> {
    inner: Element<'a>,
}
impl<'a> AttrBuilder<'a> {
    /// Create a `AttrBuilder`
    fn new() -> Self {
        AttrBuilder { inner: single!("") }
    }
    /// Add one whole attribute
    pub fn attr_whole(&mut self, a: impl Display + 'a) -> &mut Self {
        self.inner.append(single!(a));
        self
    }
    /// Add one attribute.
    pub fn attr(&mut self, name: impl Display + 'a, b: impl Display + 'a) -> &mut Self {
        self.inner.append(single!(formatm!("{}=\"{}\" ", name, b)));
        self
    }
    /// Finish creating a `Attr`
    pub fn build(&mut self) -> Attr<'a> {
        let mut k = single!("");
        core::mem::swap(&mut k, &mut self.inner);
        Attr { inner: k }
    }
}

/// Create the attribute for a svg polyline or polygon.
pub struct PathBuilder<'a> {
    inner: Element<'a>,
}
impl<'a> PathBuilder<'a> {
    /// Create a `PathBuilder`
    fn new() -> Self {
        PathBuilder {
            inner: single!("d=\""),
        }
    }

    /// Add one path command.
    pub fn add<F: fmt::Display + 'a>(&mut self, val: PathCommand<F>) -> &mut Self {
        self.inner
            .append(single!(moveable_format(move |f| val.write(f))));
        self
    }

    /// Finish creating a path.
    pub fn build(&mut self) -> Path<'a> {
        self.inner.append(single!("\""));
        let mut k = single!("");
        core::mem::swap(&mut k, &mut self.inner);
        Path { inner: k }
    }
}

/// The finished product of [`PointsBuilder`]
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
    /// Create a `PointsBuilder`
    fn new() -> Self {
        PointsBuilder {
            inner: single!("points=\""),
        }
    }

    /// Add one point to the list.
    pub fn add(&mut self, x: impl fmt::Display + 'a, y: impl fmt::Display + 'a) -> &mut Self {
        self.inner.append(single!(formatm!("{},{} ", x, y)));
        self
    }

    /// Finish creating the point list.
    pub fn build(&mut self) -> Points<'a> {
        self.inner.append(single!("\""));
        let mut k = single!("");
        core::mem::swap(&mut k, &mut self.inner);
        Points { inner: k }
    }
}

/// Shorthand for `moveable_format(move |w|write!(w,...))`
/// Similar to `format_args!()` except has a more flexible lifetime.
#[macro_export]
macro_rules! formatm {
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

/// Create a [`PathBuilder`]
pub fn path_builder<'a>() -> PathBuilder<'a> {
    PathBuilder::new()
}

/// Create a [`PointsBuilder`]
pub fn points_builder<'a>() -> PointsBuilder<'a> {
    PointsBuilder::new()
}

/// Create a [`AttrBuilder`]
pub fn attr_builder<'a>() -> AttrBuilder<'a> {
    AttrBuilder::new()
}

/// Create an element
#[macro_export]
macro_rules! elem {
    ($a:tt, $b:expr) => {
        $crate::Element::new(
            $crate::formatm!(concat!("<", $a, " {}>"), $b),
            concat!("</", $a, ">"),
        );
    };
    ($a:tt) => {
        $crate::Element::new(concat!("<", $a, ">"), concat!("</", $a, ">"));
    };
}

/// Create a single tag element
#[macro_export]
macro_rules! single {
    ($a:tt, $b:expr) => {
        $crate::Element::new($crate::formatm!(concat!("<", $a, " {}/>"), $b), "");
    };
    ($a:expr) => {
        $crate::Element::new($a, "");
    };
}
