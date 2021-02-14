

use core::fmt::Write;
use core::fmt::Error;


pub struct Element<'a,T:Write>{
    writer:&'a mut T,
    tag:&'a str,
    level:usize
}

impl<'a,T:Write> Drop for Element<'a,T>{
    fn drop(&mut self){
        let tabs="\t".repeat(self.level);
        
        //Silently fail inside of drop
        let _ = self.writer.write_str(&format!("{}</{}>\n",tabs,self.tag));
        
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
    pub fn tcut(mut self){
        self.writer.take().unwrap().write_str("/>\n").unwrap();
    }
    
    pub fn tcut_short(mut self){
        self.writer.take().unwrap().write_str(">\n").unwrap();
    }

    pub fn app(mut self,s:&str)->Self{
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

    
    pub fn tend(mut self)->Element<'a,T>{
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
    pub fn new(writer:&'a mut T,tag:&'a str)->TagBuilder<'a,T>{
        write_start_tag(writer,0,tag).unwrap();
        TagBuilder::new(writer,tag,0)
    }

    pub fn write_str(&mut self,s:&str){
        self.writer.write_str(s).unwrap()
    }

    pub fn get_writer(&mut self)->&mut T{
        self.writer
    }

    pub fn tag<'b>(&'b mut self,tag:&'b str)->TagBuilder<'b,T>{
        write_start_tag(self.writer,self.level+1,tag).unwrap();
        TagBuilder::new(self.writer,tag,self.level+1)
    }
    

}

fn write_start_tag<T:Write>(writer:&mut T,num_tabs:usize,tag:&str)->Result<(),Error>{
    for _ in 0..num_tabs{
        writer.write_char('\t')?
    }
    writer.write_char('<')?;
    writer.write_str(tag)?;
    
    Ok(())
}




//TODO add option to not add newline chars.
