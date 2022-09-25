use std::fmt;

pub struct MyWrite<'a>(pub &'a mut dyn fmt::Write);

impl fmt::Write for MyWrite<'_> {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        self.0.write_str(s)
    }

    fn write_char(&mut self, c: char) -> Result<(), fmt::Error> {
        self.0.write_char(c)
    }
    fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> Result<(), fmt::Error> {
        self.0.write_fmt(args)
    }
}

pub trait Attr {
    fn render(self, w: &mut MyWrite) -> std::fmt::Result;
    fn chain<R: Attr>(self, other: R) -> AttrChain<Self, R>
    where
        Self: Sized,
    {
        AttrChain {
            first: self,
            second: other,
        }
    }
}

impl Attr for () {
    fn render(self, _: &mut MyWrite) -> std::fmt::Result {
        Ok(())
    }
}

#[derive(Copy, Clone)]
pub struct AttrChain<A, B> {
    first: A,
    second: B,
}
impl<A: Attr, B: Attr> Attr for AttrChain<A, B> {
    fn render(self, w: &mut MyWrite) -> std::fmt::Result {
        let AttrChain { first, second } = self;
        use fmt::Write;
        first.render(w)?;
        w.write_str(" ")?;
        second.render(w)
    }
}

// pub fn attr<A: fmt::Display, B: fmt::Display>(first: A, second: B) -> SingleAttr<A, B> {
//     SingleAttr { first, second }
// }

// #[derive(Copy, Clone)]
// pub struct SingleAttr<A, B> {
//     first: A,
//     second: B,
// }
impl<A: fmt::Display, B: fmt::Display> Attr for (A, B) {
    fn render(self, w: &mut MyWrite) -> std::fmt::Result {
        let (first, second) = self;
        use fmt::Write;
        write!(crate::escape_guard(&mut *w), " {}", first)?;
        w.write_str("=\"")?;
        write!(crate::escape_guard(&mut *w), "{}", second)?;
        w.write_str("\"")
    }
}

pub trait RenderTail {
    fn render(self, w: &mut MyWrite) -> std::fmt::Result;
}

impl RenderTail for () {
    fn render(self, _: &mut MyWrite) -> std::fmt::Result {
        Ok(())
    }
}

pub fn dyn_elem<F>(func: F) -> DynamicElem<F>
where
    F: FnOnce(&mut crate::ElemWriter<&mut MyWrite>) -> fmt::Result,
{
    DynamicElem { func }
}

#[derive(Copy, Clone)]
pub struct DynamicElem<F> {
    func: F,
}
impl<F> RenderElem for DynamicElem<F>
where
    F: FnOnce(&mut crate::ElemWriter<&mut MyWrite>) -> fmt::Result,
{
    type Tail = ();

    fn render_head(self, w: &mut MyWrite) -> Result<Self::Tail, fmt::Error> {
        (self.func)(&mut crate::new(w))?;
        Ok(())
    }
}

pub trait RenderElem {
    type Tail: RenderTail;
    fn render_head(self, w: &mut MyWrite) -> Result<Self::Tail, fmt::Error>;

    fn render_with<W: fmt::Write>(self, mut w: W) -> fmt::Result
    where
        Self: Sized,
    {
        self.render_all(&mut MyWrite(&mut w))
    }
    /// Render head and tail.
    fn render_all(self, w: &mut MyWrite) -> fmt::Result
    where
        Self: Sized,
    {
        let next = self.render_head(w)?;
        next.render(w)
    }

    /// Render all of Self and head of other, store tail of other.
    fn chain<R: RenderElem>(self, other: R) -> Chain<Self, R>
    where
        Self: Sized,
    {
        Chain {
            top: self,
            bottom: other,
        }
    }

    /// Render head of Self, and all of other, store tail of self.
    fn append<R: RenderElem>(self, bottom: R) -> Append<Self, R>
    where
        Self: Sized,
    {
        Append { top: self, bottom }
    }

    // fn put_raw<D: fmt::Display>(self, display: D) -> Disp<Self, D>
    // where
    //     Self: Sized,
    // {
    //     Disp {
    //         inner: self,
    //         display,
    //     }
    // }
}

pub fn raw<D: fmt::Display>(data: D) -> Raw<D> {
    Raw { data }
}
pub struct Raw<D> {
    data: D,
}

impl<D: fmt::Display> RenderElem for Raw<D> {
    type Tail = ();
    fn render_head(self, w: &mut MyWrite) -> Result<Self::Tail, fmt::Error> {
        use std::fmt::Write;
        //TODO write one global function
        write!(crate::escape_guard(w), " {}", self.data)?;
        Ok(())
    }
}

// #[derive(Copy, Clone)]
// pub struct Disp<R, D> {
//     inner: R,
//     display: D,
// }

// impl<R: RenderElem, D: fmt::Display> RenderElem for Disp<R, D> {
//     type Tail = R::Tail;
//     fn render_head(self, w: &mut MyWrite) -> Result<Self::Tail, fmt::Error> {
//         let Disp { inner, display } = self;
//         let tail = inner.render_head(w)?;
//         use std::fmt::Write;
//         //TODO write one global function
//         write!(crate::escape_guard(w), " {}", display)?;
//         Ok(tail)
//     }
// }

#[derive(Copy, Clone)]
pub struct Append<A, B> {
    top: A,
    bottom: B,
}

impl<A: RenderElem, B: RenderElem> RenderElem for Append<A, B> {
    type Tail = A::Tail;
    fn render_head(self, w: &mut MyWrite) -> Result<Self::Tail, fmt::Error> {
        let Append { top, bottom } = self;
        let tail = top.render_head(w)?;
        bottom.render_all(w)?;
        Ok(tail)
    }
}

impl<I: IntoIterator<Item = R>, R: RenderElem> RenderElem for I {
    type Tail = ();
    fn render_head(self, w: &mut MyWrite) -> Result<Self::Tail, fmt::Error> {
        for i in self {
            i.render_all(w)?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone)]
pub struct Chain<A, B> {
    top: A,
    bottom: B,
}

impl<A: RenderElem, B: RenderElem> RenderElem for Chain<A, B> {
    type Tail = B::Tail;
    fn render_head(self, w: &mut MyWrite) -> Result<Self::Tail, fmt::Error> {
        let Chain { top, bottom } = self;
        top.render_all(w)?;
        bottom.render_head(w)
    }
}

pub struct Single<D, A> {
    tag: D,
    attr: A,
}

impl<D: fmt::Display, A: Attr> Single<D, A> {
    pub fn with_attr<AA: Attr>(self, attr: AA) -> Single<D, AA> {
        Single {
            tag: self.tag,
            attr,
        }
    }
}
impl<D: fmt::Display, A: Attr> RenderElem for Single<D, A> {
    type Tail = ();
    fn render_head(self, w: &mut MyWrite) -> Result<Self::Tail, fmt::Error> {
        use fmt::Write;
        let Single { tag, attr } = self;
        w.write_char('<')?;
        write!(crate::escape_guard(&mut *w), "{}", tag)?;
        w.write_char(' ')?;
        attr.render(w)?;
        w.write_str(" />")?;
        Ok(())
    }
}

pub fn single<D: fmt::Display>(tag: D) -> Single<D, ()> {
    Single { tag: tag, attr: () }
}

pub fn elem<D: fmt::Display>(tag: D) -> Elem<D, ()> {
    Elem { tag, attr: () }
}

#[derive(Copy, Clone)]
pub struct ElemTail<D> {
    tag: D,
}

impl<D: fmt::Display> RenderTail for ElemTail<D> {
    fn render(self, mut w: &mut MyWrite) -> std::fmt::Result {
        crate::write_tail(&mut w, &self.tag)
    }
}

#[derive(Copy, Clone)]
pub struct Elem<D, A> {
    tag: D,
    attr: A,
}

impl<D: fmt::Display, A: Attr> Elem<D, A> {
    pub fn with_attr<AA: Attr>(self, attr: AA) -> Elem<D, AA> {
        Elem {
            tag: self.tag,
            attr,
        }
    }
}
impl<D: fmt::Display, A: Attr> RenderElem for Elem<D, A> {
    type Tail = ElemTail<D>;
    fn render_head(self, w: &mut MyWrite) -> Result<Self::Tail, fmt::Error> {
        let Elem { tag, attr } = self;

        use fmt::Write;
        w.write_char('<')?;
        write!(crate::escape_guard(&mut *w), "{}", tag)?;
        w.write_char(' ')?;
        attr.render(w)?;
        w.write_str(" >")?;

        Ok(ElemTail { tag })
    }
}

#[test]
fn test_svg() {
    
    let potato = elem("potato");
    let chicken = elem("chicken").with_attr(("a", "a").chain(("b", "b")));
    let html = elem("html").with_attr( ("a", "a"));

    let k = html.append(chicken.chain(potato));
    //let k=html.append(potato).append(chicken);
    //let html = elem("html", crate::empty_attr);

    let mut w = crate::upgrade_write(std::io::stdout());
    k.render_all(&mut MyWrite(&mut w)).unwrap();
    println!();
}

#[macro_export]
macro_rules! attrs {
    ($a:expr)=>{
        $a
    };
    ( $a:expr,$( $x:expr ),* ) => {
        {
            use $crate::render::Attr;
            let mut a=$a;
            $(
                let a=a.chain($x);
            )*

            a
        }
    };
}
