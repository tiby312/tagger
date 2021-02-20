use super::*;
pub struct PolyLineBuilder<'a, 'b, T> {
    inner: &'a mut AttrBuilder<'b, T>,
}
impl<'a, 'b, T: Write> PolyLineBuilder<'a, 'b, T> {
    pub fn new(inner: &'a mut AttrBuilder<'b, T>) -> Result<Self, fmt::Error> {
        write!(inner.inner, " points=\"")?;
        Ok(PolyLineBuilder { inner })
    }
    pub fn add_point(&mut self, point: [f32; 2]) -> Result<&mut Self, fmt::Error> {
        write!(self.inner.inner, "{},{} ", point[0], point[1])?;
        Ok(self)
    }
    pub fn finish(&'a mut self) -> Result<&'a mut AttrBuilder<'b, T>, fmt::Error> {
        write!(self.inner.inner, "\"")?;
        Ok(self.inner)
    }
}

pub struct PathBuilder<'a, 'b, T> {
    inner: &'a mut AttrBuilder<'b, T>,
}
impl<'a, 'b, T: Write> PathBuilder<'a, 'b, T> {
    pub fn new(inner: &'a mut AttrBuilder<'b, T>) -> Result<Self, fmt::Error> {
        write!(inner.inner, " d=\"")?;
        Ok(PathBuilder { inner })
    }
    pub fn move_to(&mut self, point: [impl fmt::Display; 2]) -> Result<&mut Self, fmt::Error> {
        write!(self.inner.inner, "M {} {} ", point[0], point[1])?;
        Ok(self)
    }
    pub fn line_to(&mut self, point: [impl fmt::Display; 2]) -> Result<&mut Self, fmt::Error> {
        write!(self.inner.inner, "L {} {} ", point[0], point[1])?;
        Ok(self)
    }
    pub fn close(&mut self) -> Result<&mut Self, fmt::Error> {
        write!(self.inner.inner, "z")?;
        Ok(self)
    }
    pub fn finish(&'a mut self) -> Result<&'a mut AttrBuilder<'b, T>, fmt::Error> {
        write!(self.inner.inner, "\"")?;
        Ok(self.inner)
    }
}
