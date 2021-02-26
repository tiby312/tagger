//!
//! svg related building blocks
//!
use super::*;
/// Create the attribute for a svg polyline. Used by [`WriteAttr::polyline_data`].
pub struct PolyLineBuilder<'a, T: Write> {
    inner: &'a mut T,
}
impl<'a, T: Write> PolyLineBuilder<'a, T> {
    pub(super) fn new(inner: &'a mut T) -> Result<Self, fmt::Error> {
        write!(inner, " points=\"")?;
        Ok(PolyLineBuilder { inner })
    }
    pub fn add_point(&mut self, point: [impl fmt::Display; 2]) -> Result<&mut Self, fmt::Error> {
        write!(self.inner, "{},{} ", point[0], point[1])?;
        Ok(self)
    }
    pub(super) fn finish(&'a mut self) -> Result<&'a mut T, fmt::Error> {
        write!(self.inner, "\"")?;
        Ok(self.inner)
    }
}

///
/// Construct and Write a SVG path's data.
///
/// following: [w3 spec](https://www.w3.org/TR/SVG/paths.html#PathDataGeneralInformation)
///
pub enum PathCommand<F:fmt::Display>{
    /// move to
    M(F,F),
    /// relative move to
    M_(F,F),
    /// line to
    L(F,F),
    /// relative line to
    L_(F,F),
    /// horizontal to
    H(F),
    /// relative horizontal to
    H_(F),
    /// vertical to
    V(F),
    /// relative vertical to
    V_(F),
    /// curve to
    C(F,F,F,F,F,F),
    /// relative curve to
    C_(F,F,F,F,F,F),
    /// shorthand curve to
    S(F,F,F,F),
    /// relative shorthand curve to
    S_(F,F,F,F),
    /// quadratic bezier curve to
    Q(F,F,F,F),
    /// relative quadratic bezier curve to
    Q_(F,F,F,F),
    /// shorthand quadratic bezier curve to
    T(F,F),
    /// relative shorthand quadratic bezier curve to
    T_(F,F),
    /// elliptical arc
    A(F,F,F,F,F,F,F),
    /// relative elliptical arc
    A_(F,F,F,F,F,F,F),
}

impl<F:fmt::Display> PathCommand<F>{

    pub fn write<T:fmt::Write>(&self,writer:&mut T)->fmt::Result{
        use PathCommand::*;
        match self{
            M(x,y)=>{
                write!(writer," M {} {}",x,y)
            }
            M_(x,y)=>{
                write!(writer," m {} {}",x,y)
            }
            L(x,y)=>{
                write!(writer," L {} {}",x,y)
            }
            L_(x,y)=>{
                write!(writer," l {} {}",x,y)
            }
            H(a)=>{
                write!(writer," H {}",a)
            }
            H_(a)=>{
                write!(writer," h {}",a)
            }
            V(a)=>{
                write!(writer," V {}",a)
            }
            V_(a)=>{
                write!(writer," v {}",a)
            }
            C(x1,y1,x2,y2,x,y)=>{
                write!(writer," C {} {}, {} {}, {} {}",x1,y1,x2,y2,x,y)
            }
            C_(dx1,dy1,dx2,dy2,dx,dy)=>{
                write!(writer," c {} {}, {} {}, {} {}",dx1,dy1,dx2,dy2,dx,dy)
            }
            S(x2,y2,x,y)=>{
                write!(writer," S {},{} {} {}",x2,y2,x,y)
            }
            S_(x2,y2,x,y)=>{
                write!(writer," s {},{} {} {}",x2,y2,x,y)
            }
            Q(x1,y1,x,y)=>{

                write!(writer," Q {} {}, {} {}",x1,y1,x,y)
            }
            Q_(dx1,dy1,dx,dy)=>{
                write!(writer," q {} {}, {} {}",dx1,dy1,dx,dy)
            }
            T(x,y)=>{
                write!(writer," T {} {}",x,y)
            }
            T_(x,y)=>{
                write!(writer," t {} {}",x,y)
            }
            A(rx,ry,x_axis_rotation,large_arc_flag,sweep_flag, x,y)=>{
                write!(writer," A {} {} {} {} {} {} {}",rx,ry,x_axis_rotation,large_arc_flag,sweep_flag,x,y)
            }
            A_(rx,ry,x_axis_rotation,large_arc_flag,sweep_flag, dx,dy)=>{
                write!(writer," a {} {} {} {} {} {} {}",rx,ry,x_axis_rotation,large_arc_flag,sweep_flag,dx,dy)
            }
        }
    }
}



/// Create the attribute for a svg path. Used by [`WriteAttr::path_data`]
pub struct PathBuilder<'a, T> {
    inner: &'a mut T,
}
impl<'a, T: Write> PathBuilder<'a, T> {
    pub(super) fn new(inner: &'a mut T) -> Result<Self, fmt::Error> {
        write!(inner, " d=\"")?;
        Ok(PathBuilder { inner })
    }
    pub fn draw_z(&mut self)->Result<&mut Self,fmt::Error>{
        write!(self.inner,"Z")?;
        Ok(self)
    }
    pub fn draw<F:fmt::Display>(&mut self,val:PathCommand<F>)->Result<&mut Self,fmt::Error>{
        val.write(self.inner)?;
        Ok(self)
    }
    pub(super) fn finish(&'a mut self) -> Result<&'a mut T, fmt::Error> {
        write!(self.inner, "\"")?;
        Ok(self.inner)
    }
}
