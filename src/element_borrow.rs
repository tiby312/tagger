use super::*;


pub struct Element<'a,T:Write>{
    writer:&'a mut T,
    tag:&'a str,
    level:usize
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





impl<'a,T:Write> ElementTrait for Element<'a,T>{
    type W=T;
    fn get_writer(&mut self)->&mut T{
        &mut self.writer
    }
    fn get_level(&self)->usize{
        self.level
    }
}





pub struct TagBuilder<'a,T:Write>{
    writer:Option<&'a mut T>,
    tag:&'a str,
    level:usize,
}
impl<'a,T:Write> TagBuilderTrait for TagBuilder<'a,T>{
    type W=T;
    //Gives user access to the writer for more control
    //Before it is returned, a space is added.
    fn get_writer(&mut self)->&mut T{
        self.writer.as_mut().unwrap()
    }
}
impl<'a,T:Write> TagBuilder<'a,T>{
    pub(super) fn new(writer:&'a mut T,tag:&'a str,level:usize)->TagBuilder<'a,T>{
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