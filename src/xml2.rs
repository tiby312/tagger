use core::fmt;



use super::*;
pub struct PolyLineBuilder<'a, T:Write> {
    inner: &'a mut T,
}
impl<'a, T: Write> PolyLineBuilder<'a, T> {
    pub fn new(inner: &'a mut T) -> Result<Self, fmt::Error> {
        write!(inner, " points=\"")?;
        Ok(PolyLineBuilder { inner })
    }
    pub fn add_point(&mut self, point: [impl fmt::Display; 2]) -> Result<&mut Self, fmt::Error> {
        write!(self.inner, "{},{} ", point[0], point[1])?;
        Ok(self)
    }
    pub fn finish(&'a mut self) -> Result<&'a mut T, fmt::Error> {
        write!(self.inner, "\"")?;
        Ok(self.inner)
    }
}

pub struct PathBuilder<'a, T> {
    inner: &'a mut T,
}
impl<'a, T: Write> PathBuilder<'a, T> {
    pub fn new(inner: &'a mut T) -> Result<Self, fmt::Error> {
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
    pub fn finish(&'a mut self) -> Result<&'a mut T, fmt::Error> {
        write!(self.inner, "\"")?;
        Ok(self.inner)
    }
}



pub struct AttrBuilder2<'a,T>(&'a mut Element<T>);

impl<'a,T:Write> AttrBuilder2<'a,T>{

    pub fn build<F>(self,func:F)->Result<&'a mut Element<T>,fmt::Error>
    where for<'x,'y> F:FnOnce(&'x mut AttrBuilder<'y,T>)->Result<&'x mut AttrBuilder<'y,T>,fmt::Error>{
        let _res=func(&mut AttrBuilder{
            inner:self.0
        });

        write!(self.0,">")?;
        Ok(self.0)
    }
}



pub trait AttrTrait:Write+Sized{
    
    fn polyline_data<'b, F>(
        &'b mut self,
        func: F,
    ) -> Result<&'b mut Self, fmt::Error>
    where
        for<'x, 'y> F: FnOnce(
            &'x mut PolyLineBuilder<'y, Self>,
        ) -> Result<&'x mut PolyLineBuilder<'y, Self>, fmt::Error>,
    {
        {
            let mut p = PolyLineBuilder::new(self)?;
            func(&mut p)?;
            p.finish()?;
        }
        Ok(self)
    }


    fn path_data<'b, F>(&'b mut self, func: F) -> Result<&'b mut Self, fmt::Error>
    where
        for<'x, 'y> F: FnOnce(
            &'x mut PathBuilder<'y, Self>,
        ) -> Result<&'x mut PathBuilder<'y, Self>, fmt::Error>,
    {
        {
            let mut p = PathBuilder::new(self)?;
            func(&mut p)?;
            p.finish()?;
        }
        Ok(self)
    }

    fn with_attr(
        &mut self,
        s: &str,
        func: impl FnOnce(&mut Self) -> core::fmt::Result,
    ) -> Result<&mut Self, core::fmt::Error> {
        write!(self, " {}=", s)?;
        write!(self, "\"")?;
        func(self)?;
        write!(self, "\"")?;
        Ok(self)
    }
    fn attr(
        &mut self,
        s: &str,
        val: impl core::fmt::Display,
    ) -> Result<&mut Self, core::fmt::Error> {
        write!(self, " {}=\"{}\"", s, val)?;
        Ok(self)
    }
}


pub struct AttrBuilder<'a, T> {
    inner: &'a mut Element<T>,
}

impl<'a,T:fmt::Write> fmt::Write for AttrBuilder<'a,T> {
    fn write_str(&mut self,s:&str) -> fmt::Result {
        self.inner.write_str(s)
    }
}
impl<'a,T:fmt::Write> AttrTrait for AttrBuilder<'a,T>{}






pub struct Element<T> {
    writer: T,
}

impl<T:fmt::Write> fmt::Write for Element<T> {
    fn write_str(&mut self,s:&str) -> fmt::Result {
        self.writer.write_str(s)
    }
}
impl<T: fmt::Write> Element<T> {
    pub fn new(writer: T) -> Self {
        Element { writer }
    }

    pub fn ok(&mut self)->Result<&mut Self,fmt::Error>{
        Ok(self)
    }

    pub fn single<F>(&mut self,tags:[&str;3],func:F )->fmt::Result
    where
        for<'x, 'y> F:
            FnOnce(&'x mut AttrBuilder<'y, T>) -> Result<&'x mut AttrBuilder<'y, T>, fmt::Error>,{
        let [start,tag,end]=tags;
        write!(self.writer,"{}{}",start,tag)?;
        func(&mut AttrBuilder {
            inner: self,
        })?;
        write!(self.writer,"{}",end)
    }
    

    pub fn elem2<F>(
        &mut self,
        tag:&str,
        func:F)->fmt::Result
        where for<'x> F:FnOnce(AttrBuilder2<'x,T>)->Result<&'x mut Element<T>,fmt::Error>{
            
            write!(self.writer, "<{} ", tag)?;
            let attr=AttrBuilder2(self);

            //write!(self.writer, ">")?;
        
            //TODO check that we received right thing??
            let e=func(attr)?;
            
            write!(self.writer, "</{}>", tag)   
    }
    /*
    pub fn elem<F>(
        &mut self,
        tag: &str,
        attr: F,
        func: impl FnOnce(&mut Element<T>) -> fmt::Result,
    ) -> fmt::Result
    where
        for<'x, 'y> F:
            FnOnce(&'x mut AttrBuilder<'y, T>) -> Result<&'x mut AttrBuilder<'y, T>, fmt::Error>,
    {
        {
            write!(self.writer, "<{} ", tag)?;
            attr(&mut AttrBuilder {
                inner: &mut self.writer,
            })?;
            write!(self.writer, ">")?;
        }
        func(self)?;
        write!(self.writer, "</{}>", tag)
    }
    */
}
