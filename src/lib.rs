use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

/// Each function will only be run exactly once!!!!
trait Elem{
    fn header(&self, f: &mut Formatter<'_>) -> fmt::Result;
    fn body(&self, f: &mut Formatter<'_>) -> fmt::Result;
    fn end(&self, f: &mut Formatter<'_>) -> fmt::Result;
}






struct ElementWrapper<T:Elem,J:Elem>{
    a:T,
    b:J
}

impl<T:Elem,J:Elem> Elem for ElementWrapper<T,J>{

    fn header(&self, f: &mut Formatter<'_>) -> fmt::Result{
        self.a.header(f)
    }
    fn body(&self, f: &mut Formatter<'_>) -> fmt::Result{
        self.a.body(f)?;
        self.b.header(f)?;
        self.b.body(f)?;
        self.b.end(f)
    }
    fn end(&self, f: &mut Formatter<'_>) -> fmt::Result{
        self.a.end(f)
    }
}


struct Empty;
impl Elem for Empty{
    fn header(&self, _: &mut Formatter<'_>) -> fmt::Result{
        Ok(())
    }
    fn body(&self, _: &mut Formatter<'_>) -> fmt::Result{
        Ok(())
    }
    fn end(&self, _: &mut Formatter<'_>) -> fmt::Result{
        Ok(())
    }
}


pub struct BoxedElement<'a>{
    inner:InnerElem<'a>
}

struct InnerElem<'a>{
    inner:Box<dyn Elem+'a>
}

impl<'a> InnerElem<'a>{
    fn new(inner:impl Elem+'a)->Self{
        InnerElem{inner:Box::new(inner)}
    }
}
impl<'a> Elem for InnerElem<'a>{
    fn header(&self, f: &mut Formatter<'_>) -> fmt::Result{
        self.inner.header(f)
    }
    fn body(&self, f: &mut Formatter<'_>) -> fmt::Result{
        self.inner.body(f)
    }
    fn end(&self, f: &mut Formatter<'_>) -> fmt::Result{
        self.inner.end(f)
    }
}

impl<'a> Elem for BoxedElement<'a>{
    fn header(&self, f: &mut Formatter<'_>) -> fmt::Result{
        self.inner.inner.header(f)
    }
    fn body(&self, f: &mut Formatter<'_>) -> fmt::Result{
        self.inner.inner.body(f)
    }
    fn end(&self, f: &mut Formatter<'_>) -> fmt::Result{
        self.inner.inner.end(f)
    }
}
impl<'a> Display for BoxedElement<'a>{
    fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result{
        self.header(f)?;
        self.body(f)?;
        self.end(f)
    }
}
impl<'a> BoxedElement<'a>{
    pub fn append(&mut self,b:BoxedElement<'a>){
        let mut a=InnerElem::new(Empty);
        core::mem::swap(&mut a,&mut self.inner);
        let e=ElementWrapper{
            a,
            b
        };

        self.inner=InnerElem{inner:Box::new(e)};
        //core::mem::swap(&mut self.inner.inner,&mut j);
        //self.inner=Some(InnerElem{inner:Box::new(e)});
    }
    
}


pub fn element<'a,A:Display+'a,B:Display+'a,C:Display+'a>(header:A,end:C,body:B)->BoxedElement<'a>{

    struct Element<A,B,C>{
        header:A,
        body:B,
        end:C
    }
    
    impl<A:Display,B:Display,C:Display> Element<A,B,C>{
        fn box_self<'a>(self)->BoxedElement<'a>
            where A:'a,B:'a,C:'a{
            BoxedElement{inner:InnerElem{inner:Box::new(self)}}
        }
    }
    
    impl<A:Display,B:Display,C:Display> Elem for Element<A,B,C>{
        fn header(&self, f: &mut Formatter<'_>) -> fmt::Result{
            write!(f,"{}",self.header)
        }
        fn body(&self, f: &mut Formatter<'_>) -> fmt::Result{
            write!(f,"{}",self.body)
        }
        fn end(&self, f: &mut Formatter<'_>) -> fmt::Result{
            write!(f,"{}",self.end)
        }
    }

    Element{
        header,
        body,
        end
    }.box_self()
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


/// Create the attribute for a svg polyline or polygon.
pub struct PathBuilder<'a> {
    inner: BoxedElement<'a>,
}
impl<'a> PathBuilder<'a> {
    pub fn new() -> Self {
        PathBuilder{inner:one_thing!("d=\"")}
    }
    pub fn draw_z(&mut self)->&mut Self{
        self.inner.append(one_thing!("Z"));
        self
    }
    pub fn draw<F: fmt::Display+'a>(&mut self, val: PathCommand<F>) -> &mut Self {
        self.inner.append(one_thing!(moveable_format(move |f|val.write(f) )));
        self
    }
    
    pub fn finish(&mut self) -> &mut Self {
        self.inner.append(one_thing!("\""));
        self
    }


}
impl<'a> Display for PathBuilder<'a>{
    fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result{
        self.inner.fmt(f)
    }
}


/// Create the attribute for a svg polyline or polygon.
pub struct PointsBuilder<'a> {
    inner: BoxedElement<'a>,
}
impl<'a> PointsBuilder<'a> {
    pub fn new() -> Self {
        PointsBuilder{inner:one_thing!("points=\"")}
    }
    pub fn add(
        &mut self,
        x: impl fmt::Display+'a,
        y: impl fmt::Display+'a,
    ) -> &mut Self {
        self.inner.append(one_thing!(move_format!("{},{} ", x, y)));
        self
    }

    pub fn finish(&mut self) -> &mut Self {
        self.inner.append(one_thing!("\""));
        self
    }
}
impl<'a> Display for PointsBuilder<'a>{
    fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result{
        self.inner.fmt(f)
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
macro_rules! one_thing {
    // `()` indicates that the macro takes no argument.
    ($a:expr) => {
        // The macro will expand into the contents of this block.
        element("","",$a);
    };
}

#[macro_export]
macro_rules! empty_elem {
    // `()` indicates that the macro takes no argument.
    ($a:tt) => {
        // The macro will expand into the contents of this block.
        element(concat!("<",$a,">"),concat!("</",$a,">"),"");
    };
}
