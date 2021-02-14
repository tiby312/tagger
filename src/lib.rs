

use core::fmt::Write;


pub struct Element<'a,T:Write>{
    writer:&'a mut T,
    tag:&'a str,
    level:usize
}

///Starting point.
///Doesnt actually write anything out.
pub fn root<T:Write>(writer:&mut T)->Element<T>{
    Element{
        writer,
        tag:"",
        level:0
    }
}

impl<'a,T:Write> Drop for Element<'a,T>{
    fn drop(&mut self){
        
        if !self.tag.is_empty(){
            for _ in 0..self.level-1{
                let _ =self.writer.write_char('\t');
            }    
            let _ =self.writer.write_str("</");
            let _ =self.writer.write_str(self.tag);
            let _ =self.writer.write_str(">\n");
        }
    }
}



pub struct TagBuilder<'a,T:Write>{
    writer:Option<&'a mut T>,
    tag:&'a str,
    level:usize,
}
impl<'a,T:Write> TagBuilder<'a,T>{
    fn new(writer:&'a mut T,tag:&'a str,level:usize)->TagBuilder<'a,T>{
        TagBuilder{
            writer:Some(writer),
            tag,
            level,
        }
    }

    pub fn path_data(&mut self)->PathData<T>{
        PathData::new(self.writer.as_mut().unwrap())
    }

    pub fn polyline_data(&mut self)->PolyLine<T>{
        PolyLine::new(self.writer.as_mut().unwrap())
    }

    pub fn empty(mut self){
        self.writer.take().unwrap().write_str("/>\n").unwrap();
    }
    
    pub fn empty_no_slash(mut self){
        self.writer.take().unwrap().write_str(">\n").unwrap();
    }

    pub fn append(mut self,s:&str)->Self{
        let w=self.writer.as_mut().unwrap();
        w.write_char(' ').unwrap();
        w.write_str(s).unwrap();
        self
    }


    pub fn set<F:core::fmt::Display>(mut self,attr:&str,val:F)->Self{
        let w=self.writer.as_mut().unwrap();
        w.write_char(' ').unwrap();
        w.write_str(attr).unwrap();
        w.write_str(" = ").unwrap();
        write!(w,"\"{}\"",val).unwrap();
        self
    }

    //Gives user access to the writer for more control
    //Before it is returned, a space is added.
    pub fn get_writer(&mut self)->&mut T{
        self.writer.as_mut().unwrap()
    }

    
    pub fn end(mut self)->Element<'a,T>{
        let writer=self.writer.take().unwrap();
        
        writer.write_str(">\n").unwrap();
        Element{
            writer,
            tag:self.tag,
            level:self.level
        }
    }
}
impl<'a,T:Write> Drop for TagBuilder<'a,T>{
    fn drop(&mut self){
        if let Some(writer)=self.writer.take(){
            
            let _ = writer.write_str(">\n");
        }
    }
}

impl<'a,T:Write> Element<'a,T>{
    

    pub fn write_str(&mut self,s:&str){
        self.writer.write_str(s).unwrap()
    }

    pub fn get_writer(&mut self)->&mut T{
        self.writer
    }

    pub fn declaration(&mut self,data:&str){
        for _ in 0..self.level{
            self.writer.write_char('\t').unwrap();
        }
        self.writer.write_str("<!").unwrap();
        self.writer.write_str(data).unwrap();
        self.writer.write_str(">\n").unwrap();
    }
    pub fn comment(&mut self,data:&str){
        for _ in 0..self.level{
            self.writer.write_char('\t').unwrap();
        }
        self.writer.write_str("<-- ").unwrap();
        self.writer.write_str(data).unwrap();
        self.writer.write_str(" -->\n").unwrap();
    }


    pub fn tag_build<'b>(&'b mut self,tag:&'b str)->TagBuilder<'b,T>{
        assert!(!tag.is_empty(),"Can't have an empty string for a tag");
        for _ in 0..self.level{
            self.writer.write_char('\t').unwrap();
        }
        self.writer.write_char('<').unwrap();
        self.writer.write_str(tag).unwrap();
        TagBuilder::new(self.writer,tag,self.level+1)
    }
}



pub struct PolyLine<'a,T:Write>{
    writer:&'a mut T
}
impl<'a,T:Write> Drop for PolyLine<'a,T>{
    fn drop(&mut self){
        let _ = write!(self.writer,"\"");
    }
}
impl<'a,T:Write> PolyLine<'a,T>{
    fn new(writer:&'a mut T)->PolyLine<'a,T>{
        write!(writer," points=\"").unwrap();
        PolyLine{writer}
    }
    pub fn point(&mut self,point:[f32;2]){
        write!(self.writer,"{},{} ",point[0],point[1]).unwrap();
    }
}

pub struct PathData<'a,T:Write>{
    writer:&'a mut T
}
impl<'a,T:Write> Drop for PathData<'a,T>{
    fn drop(&mut self){
        let _ = write!(self.writer,"\"").unwrap();
    }
}
impl<'a,T:Write> PathData<'a,T>{
    fn new(writer:&'a mut T)->Self{
        write!(writer," d=\"").unwrap();
        PathData{writer}
    }
    pub fn close(&mut self){
        write!(self.writer,"z").unwrap();
    }
    pub fn move_to(&mut self,point:[f32;2])->&mut Self{
        write!(self.writer,"M {} {} ",point[0],point[1]).unwrap();
        self
    }
    pub fn line_to(&mut self,point:[f32;2])->&mut Self{
        write!(self.writer,"L {} {} ",point[0],point[1]).unwrap();
        self
    }
}



use std::*;

pub struct Adaptor<T> {
    inner:  T,
    error: Result<(),io::Error>,
}

///Upgrade a std::io::Write to be a std::fmt::Write
pub fn upgrade_writer<T:std::io::Write>(inner:T)->Adaptor<T>{
    Adaptor{
        inner,
        error:Ok(())
    }
}
impl<T: io::Write> fmt::Write for Adaptor<T> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        match self.inner.write_all(s.as_bytes()) {
            Ok(()) => Ok(()),
            Err(e) => {
                self.error = Err(e);
                Err(fmt::Error)
            }
        }
    }
}