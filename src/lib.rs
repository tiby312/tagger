

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

    pub fn attr<F:core::fmt::Display>(mut self,attr:&str,val:F)->Self{
        let w=self.writer.as_mut().unwrap();
        w.write_char(' ').unwrap();
        w.write_str(attr).unwrap();
        w.write_str(" = ").unwrap();
        write!(w,"\"{}\"",val).unwrap();
        self
    }

    //Gives user access to the writer for more control
    //Before it is returned, a space is added.
    pub fn writer(&mut self)->&mut T{
        let w=self.writer.as_mut().unwrap();
        w.write_char(' ').unwrap();
        w
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