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

/// The tagger prelude
pub mod prelude {
    pub use crate::element;
    pub use crate::single_element;
    pub use crate::path;
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

/*
/// The finished product of [`PointsBuilder`]
pub struct Points<'a> {
    inner: Element<'a>,
}

impl<'a> Display for Points<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.display().fmt(f)
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
            inner: Element::one_new(""),
        }
    }

    /// Add one point to the list.
    pub fn add(&mut self, x: impl fmt::Display + 'a, y: impl fmt::Display + 'a) -> &mut Self {
        self.inner.append(formatm!("{},{} ", x, y));
        self
    }

    /// Finish creating the point list.
    pub fn build(&mut self) -> Points<'a> {
        let mut k = Element::one_new("");
        core::mem::swap(&mut k, &mut self.inner);
        Points { inner: k }
    }
}
*/

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




pub struct Connector<'a, T> {
    pub writer: &'a mut T,
    pub inner: &'a str,
}
impl<'a, T: fmt::Write> Connector<'a, T> {
    pub fn build(mut self, a: impl FnOnce(&mut T) -> fmt::Result) -> fmt::Result {
        a(&mut self.writer)?;
        write!(self.writer, "</{}>", self.inner)
    }
}

#[macro_export]
macro_rules! single_element {
    ($w:expr,$a:tt ) => (
        {
            use std::fmt::Write;
            write!($w,concat!("<",$a ))?;

            write!($w,"/>")?;

        }
    );
    ($w:expr,$a:tt,$($x:expr),* ) => (
        {
            use std::fmt::Write;
            write!($w,concat!("<",$a ))?;
            $(
                write!($w," {}=\"{}\"",$x.0,$x.1)?;
            )*

            write!($w,"/>")?;

        }
    )
}

#[macro_export]
macro_rules! element {
    ($w:expr,$a:tt) => (
        {
            use std::fmt::Write;
            write!($w,concat!("<",$a ))?;

            write!($w,">")?;

            $crate::Connector{writer:$w,inner:$a}

        }
    );
    ($w:expr,$a:tt,$($x:expr),* ) => (
        {
            use std::fmt::Write;
            write!($w,concat!("<",$a ))?;
            $(
                write!($w," {}=\"{}\"",$x.0,$x.1)?;
            )*

            write!($w,">")?;

            $crate::Connector{writer:$w,inner:$a}

        }
    )
}


#[macro_export]
macro_rules! path {
    ($($x:expr),* ) => (
        {
            use std::fmt::Write;
            path(|b|{
                $(
                    b.add($x)?;
                )*
                Ok(())
            })

        }
    )
}

pub struct PathBuilder<T> {
    writer: T,
}
impl<T:fmt::Write> PathBuilder<T> {
    pub fn add(&mut self, command: crate::PathCommand<impl fmt::Display>) -> fmt::Result {
        command.write(&mut self.writer)
    }
}
pub fn path(a: impl Fn(&mut PathBuilder<&mut fmt::Formatter>)->fmt::Result) -> impl fmt::Display {
    moveable_format(move |writer|{
        let mut p=PathBuilder{writer};
        a(&mut p)
    })
}

pub struct Adaptor<T> {
    pub inner: T,
    pub error: Result<(), std::io::Error>,
}

///Update a std::io::Write to be a std::fmt::Write
pub fn upgrade_write<T: std::io::Write>(inner: T) -> Adaptor<T> {
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
