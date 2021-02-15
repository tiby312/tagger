use super::*;

pub struct TagBuilderFlat<T:Write>{
    pub(super) tag:String,
    pub(super) inner:FlatElement<T>
}
impl<T:Write> TagBuilderFlat<T>{
    pub fn set<F:core::fmt::Display>(mut self,attr:&str,val:F)->Self{
        let w=&mut self.inner.writer;
        w.write_char(' ').unwrap();
        w.write_str(attr).unwrap();
        w.write_str(" = ").unwrap();
        write!(w,"\"{}\"",val).unwrap();
        self
    }
    pub fn empty(mut self)->FlatElement<T>{
        self.inner.writer.write_str("/>\n").unwrap();
        self.inner
    }

    pub fn empty_no_slash(mut self)->FlatElement<T>{
        self.inner.writer.write_str(">\n").unwrap();
        self.inner
    }

    pub fn end(mut self)->FlatElement<T>{
        let writer=&mut self.inner.writer;
        writer.write_str(">\n").unwrap();
        self.inner.tags.push(self.tag);
        self.inner
    }

}


pub struct FlatElement<T:Write>{
    writer:T,
    tags:Vec<String>
}
impl<T:Write> Drop for FlatElement<T>{
    fn drop(&mut self){
        for (i,tag) in self.tags.iter_mut().enumerate().rev(){
            let _ = write_end_tag(&mut self.writer,&tag,i);
            
        }
    }
}

fn write_end_tag<T:Write>(mut writer:T,tag:&str,num_level:usize)->Result<(),core::fmt::Error>{
    for _ in 0..num_level{
        writer.write_char('\t')?;
    }
    writer.write_str("</")?;
    writer.write_str(tag)?;
    writer.write_str(">\n")?;
    Ok(())
}


impl<T:Write> ElementTrait for FlatElement<T>{
    type W=T;
    fn get_writer(&mut self)->&mut T{
        &mut self.writer
    }
    fn get_level(&self)->usize{
        self.tags.len()
    }
}
impl<T:Write> FlatElement<T>{
    pub fn new(writer:T)->Self{
        FlatElement{
            writer,
            tags:Vec::new()
        }
    }
    pub fn pop(mut self)->Self{
        let i=self.tags.len();
        if let Some(tag)=self.tags.pop(){
            let _ = write_end_tag(&mut self.writer,&tag,i-1);
        }else{
            panic!("nothing to pop");
        }
        self
    }
    pub fn tag_build_flat(mut self,tag:&str)->TagBuilderFlat<T>{
        assert!(!tag.is_empty(),"Can't have an empty string for a tag");
        for _ in 0..self.tags.len(){
            self.writer.write_char('\t').unwrap();
        }
        self.writer.write_char('<').unwrap();
        self.writer.write_str(tag).unwrap();
        TagBuilderFlat{
            inner:self,
            tag:tag.to_string()
        }
    }

    pub fn tag_build<'b>(&'b mut self,tag:&'b str)->element_borrow::TagBuilder<'b,T>{
        assert!(!tag.is_empty(),"Can't have an empty string for a tag");
        for _ in 0..self.tags.len(){
            self.writer.write_char('\t').unwrap();
        }
        self.writer.write_char('<').unwrap();
        self.writer.write_str(tag).unwrap();
        element_borrow::TagBuilder::new(&mut self.writer,tag,self.tags.len()+1)
    }
}

