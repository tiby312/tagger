use core::fmt;
use core::fmt::Write;

pub struct Single<'a, T: Write> {
    writer: &'a mut T,
}
impl<'a, T: Write> Single<'a, T> {
    pub fn new(writer: &'a mut T, ar: fmt::Arguments) -> Result<Self, fmt::Error> {
        writer.write_fmt(ar)?;
        Ok(Single { writer })
    }

    pub fn borrow<'b>(&'b mut self, a: fmt::Arguments<'_>) -> Result<Element<'b, T>, fmt::Error> {
        let w=&mut self.writer;
        w.write_fmt(a)?;
        Ok(Element {
            writer: Some(w)
        })
    }
}


pub struct ElementStack<T:Write>{
    writer:T,
    ends:Vec<String>
}

impl<T: Write> ElementStack<T> {
    pub fn new(mut writer:T,ar:fmt::Arguments,last:impl ToString)->Result<Self,fmt::Error>{
        writer.write_fmt(ar)?;

        Ok(ElementStack{
            writer,
            ends:vec!(last.to_string())
        })
    }
    
    pub fn end_last(&mut self) -> fmt::Result {
        let s=self.ends.pop().unwrap();
        self.writer.write_fmt(format_args!("{}",s))?; //TODO inefficient?
        Ok(())
    }

    pub fn write_str(&mut self, s: &str) -> fmt::Result {
        write!(self.writer, "{}", s)
    }
    pub fn get_writer(&mut self) -> &mut T {
        &mut self.writer
    }

    pub fn borrow_move(&mut self,a:fmt::Arguments,last:impl ToString)->Result<(),fmt::Error>{
        self.writer.write_fmt(a)?;
        self.ends.push(last.to_string());
        Ok(())
    }
    pub fn borrow<'b>(&'b mut self, a: fmt::Arguments) -> Result<Element<'b, T>, fmt::Error> {
        let w=&mut self.writer;
        w.write_fmt(a)?;
        Ok(Element {
            writer: Some(w),
        })
    }
    pub fn borrow_single<'b>(&'b mut self, a: fmt::Arguments) -> Result<Single<'b, T>, fmt::Error> {
        let w=&mut self.writer;
        w.write_fmt(a)?;
        Ok(Single {
            writer: w,
        })
    }
}


impl<T: Write> Drop for ElementStack<T> {
    fn drop(&mut self) {
        //Runtime checked linear types.
        //we do this to force the user to handle the result of
        //writing the last tag failing.

        if  !self.ends.is_empty()  && !std::thread::panicking() {
            //TODO print out element
            panic!("end() was not called on a element");
        }
    }
}





pub struct Element<'a, T: Write> {
    writer: Option<&'a mut T>,
}
impl<'a, T: Write> Drop for Element<'a, T> {
    fn drop(&mut self) {
        //Runtime checked linear types.
        //we do this to force the user to handle the result of
        //writing the last tag failing.
        if !self.writer.is_none() && !std::thread::panicking() {
            panic!("end() must be called on this element");
        }
    }
}
impl<'a, T: Write> Element<'a, T> {
    pub fn end(mut self, a: fmt::Arguments) -> fmt::Result {
        self.writer.take().unwrap().write_fmt(a)
    }
    pub fn new(writer: &'a mut T, ar: fmt::Arguments) -> Result<Self, fmt::Error> {
        writer.write_fmt(ar)?;
        Ok(Element {
            writer:Some(writer),
        })
    }
    pub fn write_str(&mut self, s: &str) -> fmt::Result {
        write!(self.writer.as_mut().unwrap(), "{}", s)
    }
    pub fn get_writer(&mut self) -> &mut T {
        self.writer.as_mut().unwrap()
    }
    pub fn borrow<'b>(&'b mut self, a: fmt::Arguments) -> Result<Element<'b, T>, fmt::Error> {
        let w=self.writer.as_mut().unwrap();
        w.write_fmt(a)?;
        Ok(Element {
            writer: Some(w),
        })
    }
    pub fn borrow_single<'b>(&'b mut self, a: fmt::Arguments) -> Result<Single<'b, T>, fmt::Error> {
        let w=self.writer.as_mut().unwrap();
        w.write_fmt(a)?;
        Ok(Single {
            writer: w,
        })
    }
}
