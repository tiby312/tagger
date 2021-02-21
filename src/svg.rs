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

/// Create the attribute for a svg path. Used by [`WriteAttr::path_data`]
pub struct PathBuilder<'a, T> {
    inner: &'a mut T,
}
impl<'a, T: Write> PathBuilder<'a, T> {
    pub(super) fn new(inner: &'a mut T) -> Result<Self, fmt::Error> {
        write!(inner, " d=\"")?;
        Ok(PathBuilder { inner })
    }
    pub fn move_to(&mut self, point: [impl fmt::Display; 2]) -> Result<&mut Self, fmt::Error> {
        write!(self.inner, "M {} {} ", point[0], point[1])?;
        Ok(self)
    }
    pub fn line_to(&mut self, point: [impl fmt::Display; 2]) -> Result<&mut Self, fmt::Error> {
        write!(self.inner, "L {} {} ", point[0], point[1])?;
        Ok(self)
    }
    pub fn close(&mut self) -> Result<&mut Self, fmt::Error> {
        write!(self.inner, "z")?;
        Ok(self)
    }
    pub(super) fn finish(&'a mut self) -> Result<&'a mut T, fmt::Error> {
        write!(self.inner, "\"")?;
        Ok(self.inner)
    }
}
