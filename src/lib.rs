use std::fmt;
use std::fmt::Write;

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
    fn write<T: fmt::Write>(&self, mut writer: T) -> fmt::Result {
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
    pub fn put(&mut self, command: crate::PathCommand<impl fmt::Display>) -> fmt::Result {
        command.write(escape_guard(&mut self.writer))
    }
}

///
/// Build up a list of points.
///
pub struct PointsBuilder<'a, T> {
    writer: &'a mut T,
}
impl<'a, T: fmt::Write> PointsBuilder<'a, T> {
    pub fn put(&mut self, x: impl fmt::Display, y: impl fmt::Display) -> fmt::Result {
        write!(escape_guard(&mut self.writer), "{},{} ", x, y)
    }
}

///
/// Used to wrap a `std::io::Write` to have `std::io::Write`.
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

///Update a `std::io::Write` to be a `std::fmt::Write`
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

///
/// A struct that captures a half-made element. To
/// complete building an element, `build()` must be called.
///
#[must_use]
pub struct ElementBridge<'a, T, D, K> {
    writer: &'a mut ElemWriter<T>,
    tag: D,
    pub k: K,
}
impl<'a, T: fmt::Write, D: fmt::Display, K> ElementBridge<'a, T, D, K> {
    pub fn build<J>(
        self,
        func: impl FnOnce(&mut ElemWriter<T>) -> Result<J, fmt::Error>,
    ) -> Result<J, fmt::Error> {
        let k = func(self.writer)?;
        write!(&mut self.writer.0, "{}", "</")?;
        write!(escape_guard(&mut self.writer.0), "{}", self.tag)?;
        write!(&mut self.writer.0, "{}", ">")?;
        Ok(k)
    }
}

///
/// Create attributes.
///
pub struct AttrWriter<'a, T>(&'a mut T);
impl<'a, T: fmt::Write> AttrWriter<'a, T> {
    pub fn attr(&mut self, a: impl fmt::Display, b: impl fmt::Display) -> fmt::Result {
        write!(escape_guard(&mut self.0), " {}", a)?;
        write!(&mut self.0, "{}", "=\"")?;
        write!(escape_guard(&mut self.0), "{}", b)?;
        write!(&mut self.0, "{}", "\"")
    }

    ///
    /// WARNING: The user can escape xml here and inject any xml elements.
    ///
    pub fn writer(&mut self) -> &mut T {
        &mut self.0
    }
    pub fn put_raw(&mut self, a: impl fmt::Display) -> fmt::Result {
        write!(escape_guard(&mut self.0), " {}", a)
    }

    ///
    /// WARNING: The user can escape xml here and inject any xml elements.
    ///
    pub fn put_raw_escapable(&mut self, a: impl fmt::Display) -> fmt::Result {
        write!(&mut self.0, " {}", a)
    }
    pub fn path(&mut self, a: impl FnOnce(&mut PathBuilder<T>) -> fmt::Result) -> fmt::Result {
        let mut p = PathBuilder { writer: self.0 };
        write!(p.writer, "{}", " d=\"")?;
        a(&mut p)?;
        write!(p.writer, "{}", "\"")
    }
    pub fn points(&mut self, a: impl FnOnce(&mut PointsBuilder<T>) -> fmt::Result) -> fmt::Result {
        let mut p = PointsBuilder { writer: self.0 };
        write!(&mut p.writer, "{}", " points=\"")?;
        a(&mut p)?;
        write!(&mut p.writer, "{}", "\"")
    }
}

///
/// Create elements with a start and end tag, or elements with a single tag.
///
pub struct ElemWriter<T>(T);

impl<T: fmt::Write> ElemWriter<T> {
    pub fn into_writer(self) -> T {
        self.0
    }

    ///
    /// WARNING: The user can escape xml here and inject any xml elements.
    ///
    pub fn writer(&mut self) -> &mut T {
        &mut self.0
    }

    pub fn put_raw(&mut self, a: impl fmt::Display) -> fmt::Result {
        write!(escape_guard(&mut self.0), " {}", a)
    }

    ///
    /// WARNING: The user can escape xml here and inject any xml elements.
    ///
    pub fn put_raw_escapable(&mut self, a: impl fmt::Display) -> fmt::Result {
        write!(&mut self.0, " {}", a)
    }

    pub fn single<D: fmt::Display>(
        &mut self,
        tag: D,
        func: impl FnOnce(&mut AttrWriter<T>) -> fmt::Result,
    ) -> fmt::Result {
        write!(self.0, "{}", "<")?;
        write!(escape_guard(&mut self.0), "{}", tag)?;
        write!(self.0, "{}", " ")?;
        func(&mut AttrWriter(&mut self.0))?;
        write!(self.0, "{}", " />")
    }
    pub fn elem<D: fmt::Display, K>(
        &mut self,
        tag: D,
        func: impl FnOnce(&mut AttrWriter<T>) -> Result<K, fmt::Error>,
    ) -> Result<ElementBridge<T, D, K>, fmt::Error> {
        write!(self.0, "{}", "<")?;
        write!(escape_guard(&mut self.0), "{}", tag)?;
        write!(self.0, "{}", " ")?;

        let k = func(&mut AttrWriter(&mut self.0))?;

        write!(self.0, "{}", " >")?;

        Ok(ElementBridge {
            writer: self,
            tag,
            k,
        })
    }
}

///
/// Specify no attributes needed.
/// Equivalent to writing `|_|{}`.
///
pub fn no_attr<T>() -> impl FnOnce(&mut AttrWriter<T>) -> fmt::Result {
    move |_| Ok(())
}

///
/// Writer adaptor that disallows escaping from xml.
///
pub fn escape_guard<T: std::fmt::Write>(a: T) -> EscapeGuard<T> {
    EscapeGuard::new(a)
}

/// Writer adaptor that replaces xml escaping characters with their encoded value.
///
/// Disallowed characters are `"` `'` `<` `>` `&`. characters are replaced with their equivalent from:
/// [https://dev.w3.org/html5/html-author/charref](https://dev.w3.org/html5/html-author/charref)
///
pub struct EscapeGuard<T> {
    writer: T,
    buffer: String,
}

impl<T: std::fmt::Write> EscapeGuard<T> {
    pub fn new(writer: T) -> EscapeGuard<T> {
        EscapeGuard {
            writer,
            buffer: String::new(),
        }
    }
}

impl<T: std::fmt::Write> std::fmt::Write for EscapeGuard<T> {
    fn write_str(&mut self, s: &str) -> Result<(), std::fmt::Error> {
        self.buffer.clear();
        for c in s.chars() {
            let r = match c {
                '\"' => Some("&quot;"),
                '\'' => Some("&apos;"),
                '<' => Some("&lt;"),
                '>' => Some("&gt;"),
                '&' => Some("&amp;"),
                _ => None,
            };

            if let Some(r) = r {
                self.buffer.push_str(r);
            } else {
                self.buffer.push(c);
            }
        }
        
        self.writer.write_str(&self.buffer)
    }
}
