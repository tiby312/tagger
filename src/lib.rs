use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

/// Each function will only be run exactly once!!!!
trait Elem{
    fn header(&self, f: &mut Formatter<'_>) -> fmt::Result;
    fn body(&self, f: &mut Formatter<'_>) -> fmt::Result;
    fn end(&self, f: &mut Formatter<'_>) -> fmt::Result;
}






struct ElementWrapper<T,J>{
    a:T,
    b:J
}

impl<T:Elem,J:Elem> Elem for ElementWrapper<T,J>{

    fn header(&self, f: &mut Formatter<'_>) -> fmt::Result{
        self.a.header(f)
    }
    fn body(&self, f: &mut Formatter<'_>) -> fmt::Result{
        self.a.body(f)?;
        self.b.header(f)?;
        self.b.body(f)?;
        self.b.end(f)
    }
    fn end(&self, f: &mut Formatter<'_>) -> fmt::Result{
        self.a.end(f)
    }
}



pub struct BoxedElement<'a>{
    inner:Option<InnerElem<'a>>
}

struct InnerElem<'a>{
    inner:Box<dyn Elem+'a>
}

impl<'a> Elem for InnerElem<'a>{
    fn header(&self, f: &mut Formatter<'_>) -> fmt::Result{
        self.inner.header(f)
    }
    fn body(&self, f: &mut Formatter<'_>) -> fmt::Result{
        self.inner.body(f)
    }
    fn end(&self, f: &mut Formatter<'_>) -> fmt::Result{
        self.inner.end(f)
    }
}

impl<'a> Elem for BoxedElement<'a>{
    fn header(&self, f: &mut Formatter<'_>) -> fmt::Result{
        self.inner.as_ref().unwrap().inner.header(f)
    }
    fn body(&self, f: &mut Formatter<'_>) -> fmt::Result{
        self.inner.as_ref().unwrap().inner.body(f)
    }
    fn end(&self, f: &mut Formatter<'_>) -> fmt::Result{
        self.inner.as_ref().unwrap().inner.end(f)
    }
}
impl<'a> Display for BoxedElement<'a>{
    fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result{
        self.header(f)?;
        self.body(f)?;
        self.end(f)
    }
}
impl<'a> BoxedElement<'a>{
    pub fn append(&mut self,b:BoxedElement<'a>){
        let e=ElementWrapper{
            a:self.inner.take().unwrap(),
            b:b
        };
        self.inner=Some(InnerElem{inner:Box::new(e)});
    }
    /*
    pub fn append<J:Elem+'a>(&mut self,b:J){
        let e=ElementWrapper{
            a:self.inner.take().unwrap(),
            b:b
        };
        self.inner=Some(InnerElem{inner:Box::new(e)});
    }
    */
    
}



pub fn element<'a,A:Display+'a,B:Display+'a,C:Display+'a>(header:A,end:C,body:B)->BoxedElement<'a>{

    struct Element<A,B,C>{
        header:A,
        body:B,
        end:C
    }
    
    impl<A:Display,B:Display,C:Display> Element<A,B,C>{
        fn box_self<'a>(self)->BoxedElement<'a>
            where A:'a,B:'a,C:'a{
            BoxedElement{inner:Some(InnerElem{inner:Box::new(self)})}
        }
    }
    
    impl<A:Display,B:Display,C:Display> Elem for Element<A,B,C>{
        fn header(&self, f: &mut Formatter<'_>) -> fmt::Result{
            write!(f,"{}",self.header)
        }
        fn body(&self, f: &mut Formatter<'_>) -> fmt::Result{
            write!(f,"{}",self.body)
        }
        fn end(&self, f: &mut Formatter<'_>) -> fmt::Result{
            write!(f,"{}",self.end)
        }
    }

    Element{
        header,
        body,
        end
    }.box_self()
}

