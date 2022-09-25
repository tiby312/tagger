use std::{fmt, io::Stdout};

use crate::Adaptor;

pub trait Attr {
    fn render<T: fmt::Write>(self, w: &mut T) -> std::fmt::Result;
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
    fn render<T: fmt::Write>(self, _: &mut T) -> std::fmt::Result {
        Ok(())
    }
}

pub struct AttrChain<A, B> {
    first: A,
    second: B,
}
impl<A: Attr, B: Attr> Attr for AttrChain<A, B> {
    fn render<T: fmt::Write>(self, w: &mut T) -> std::fmt::Result {
        let AttrChain { first, second } = self;
        first.render(w)?;
        w.write_str(" ")?;
        second.render(w)
    }
}

pub fn attr<A: fmt::Display, B: fmt::Display>(first: A, second: B) -> SingleAttr<A, B> {
    SingleAttr { first, second }
}
pub struct SingleAttr<A, B> {
    first: A,
    second: B,
}
impl<A: fmt::Display, B: fmt::Display> Attr for SingleAttr<A, B> {
    fn render<T: fmt::Write>(self, w: &mut T) -> std::fmt::Result {
        let SingleAttr { first, second } = self;
        use fmt::Write;
        write!(crate::escape_guard(&mut *w), " {}", first)?;
        w.write_str("=\"")?;
        write!(crate::escape_guard(&mut *w), "{}", second)?;
        w.write_str("\"")
    }
}

pub trait RenderTail {
    fn render<T: fmt::Write>(self, w: &mut T) -> std::fmt::Result;
}

impl RenderTail for () {
    fn render<T: fmt::Write>(self, _: &mut T) -> std::fmt::Result {
        Ok(())
    }
}

pub trait RenderElem {
    type Tail: RenderTail;
    fn render_head<T: fmt::Write>(self, w: &mut T) -> Result<Self::Tail, fmt::Error>;

    /// Render head and tail.
    fn render_all<T: fmt::Write>(self, w: &mut T) -> fmt::Result
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

    fn put_raw<D: fmt::Display>(self, display: D) -> Disp<Self, D>
    where
        Self: Sized,
    {
        Disp {
            inner: self,
            display,
        }
    }
}

pub struct Disp<R, D> {
    inner: R,
    display: D,
}

impl<R: RenderElem, D: fmt::Display> RenderElem for Disp<R, D> {
    type Tail = R::Tail;
    fn render_head<T: fmt::Write>(self, w: &mut T) -> Result<Self::Tail, fmt::Error> {
        let Disp { inner, display } = self;
        let tail = inner.render_head(w)?;
        use std::fmt::Write;
        //TODO write one global function
        write!(crate::escape_guard(w), " {}", display)?;
        Ok(tail)
    }
}

// pub struct RenderClosure<R, F> {
//     inner: R,
//     func: F,
// }

// impl<R: RenderElem,  F> RenderElem for RenderClosure<R, F>
// {
//     type Tail = R::Tail;
//     fn render_head<T:fmt::Write>(self, w: &mut T) -> Result<Self::Tail, fmt::Error> {
//         let RenderClosure { inner, func } = self;
//         let tail = inner.render_head(w)?;
//         func(&mut crate::new(w))?;
//         Ok(tail)
//     }
// }

pub struct Append<A, B> {
    top: A,
    bottom: B,
}

impl<A: RenderElem, B: RenderElem> RenderElem for Append<A, B> {
    type Tail = A::Tail;
    fn render_head<T: fmt::Write>(self, w: &mut T) -> Result<Self::Tail, fmt::Error> {
        let Append { top, bottom } = self;
        let tail = top.render_head(w)?;
        bottom.render_all(w)?;
        Ok(tail)
    }
}

pub struct Chain<A, B> {
    top: A,
    bottom: B,
}

impl<A: RenderElem, B: RenderElem> RenderElem for Chain<A, B> {
    type Tail = B::Tail;
    fn render_head<T: fmt::Write>(self, w: &mut T) -> Result<Self::Tail, fmt::Error> {
        let Chain { top, bottom } = self;
        top.render_all(w)?;
        bottom.render_head(w)
    }
}

// pub struct Single<D, F> {
//     tag: D,
//     attr: F,
// }

// impl<D: fmt::Display, F> RenderElem for Single<D, F>
// {
//     type Tail = ();
//     fn render_head<T:fmt::Write>(self, w: &mut T) -> Result<Self::Tail, fmt::Error> {
//         let Single { tag, attr } = self;
//         crate::write_single(w, &tag, attr)?;
//         Ok(())
//     }
// }

// pub fn single<'a, D: fmt::Display + 'a, T: fmt::Write, F>(tag: D, attr: F) -> Single<D, F>
// where
//     F: FnOnce(&mut crate::AttrWriter<T>) -> std::fmt::Result + 'a,
// {
//     Single { tag: tag, attr }
// }

pub fn elem<'a, D: fmt::Display + 'a, A: Attr>(tag: D, attr: A) -> Elem<D, A> {
    Elem { tag: tag, attr }
}

pub struct ElemTail<D> {
    tag: D,
}

impl<D: fmt::Display> RenderTail for ElemTail<D> {
    fn render<T: fmt::Write>(self, w: &mut T) -> std::fmt::Result {
        crate::write_tail(w, &self.tag)
    }
}

pub struct Elem<D, A> {
    tag: D,
    attr: A,
}

impl<D: fmt::Display, A: Attr> RenderElem for Elem<D, A> {
    type Tail = ElemTail<D>;
    fn render_head<T: fmt::Write>(self, w: &mut T) -> Result<Self::Tail, fmt::Error> {
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
    let potato = elem("potato", ());
    let chicken = elem("chicken", attr("a", "a").chain(attr("b", "b")));
    let html = elem("html", attr("a", "a"));

    let k = html.append(chicken.chain(potato));
    //let k=html.append(potato).append(chicken);
    //let html = elem("html", crate::empty_attr);

    let mut w = crate::upgrade_write(std::io::stdout());
    k.render_all(&mut w).unwrap();
    println!();
}
