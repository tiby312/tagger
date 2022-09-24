use std::fmt;
pub trait RenderTail<T: fmt::Write> {
    fn render(self, w: &mut T) -> std::fmt::Result;
}

impl<T: fmt::Write> RenderTail<T> for () {
    fn render(self, _: &mut T) -> std::fmt::Result {
        Ok(())
    }
}

pub trait RenderElem<T: fmt::Write> {
    type Tail: RenderTail<T>;
    fn render_head(self, w: &mut T) -> Result<Self::Tail, fmt::Error>;

    /// Render head and tail.
    fn render_all(self, w: &mut T) -> fmt::Result
    where
        Self: Sized,
    {
        let next = self.render_head(w)?;
        next.render(w)
    }

    /// Render all of Self and head of other, store tail of other.
    fn chain<R: RenderElem<T>>(self, other: R) -> Chain<Self, R>
    where
        Self: Sized,
    {
        Chain {
            top: self,
            bottom: other,
        }
    }

    /// Render head of Self, and all of other, store tail of self.
    fn append<R: RenderElem<T>>(self, bottom: R) -> Append<Self, R>
    where
        Self: Sized,
    {
        Append { top: self, bottom }
    }

    fn add<F>(self, func: F) -> RenderClosure<Self, F>
    where
        Self: Sized,
        F: FnOnce(&mut crate::ElemWriter<&mut T>) -> fmt::Result,
    {
        RenderClosure { inner: self, func }
    }
}

pub struct RenderClosure<R, F> {
    inner: R,
    func: F,
}

impl<R: RenderElem<T>, T: fmt::Write, F> RenderElem<T> for RenderClosure<R, F>
where
    F: FnOnce(&mut crate::ElemWriter<&mut T>) -> fmt::Result,
{
    type Tail = R::Tail;
    fn render_head(self, w: &mut T) -> Result<Self::Tail, fmt::Error> {
        let RenderClosure { inner, func } = self;
        let tail = inner.render_head(w)?;
        func(&mut crate::new(w))?;
        Ok(tail)
    }
}

pub struct Append<A, B> {
    top: A,
    bottom: B,
}

impl<A: RenderElem<T>, B: RenderElem<T>, T: fmt::Write> RenderElem<T> for Append<A, B> {
    type Tail = A::Tail;
    fn render_head(self, w: &mut T) -> Result<Self::Tail, fmt::Error> {
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

impl<A: RenderElem<T>, B: RenderElem<T>, T: fmt::Write> RenderElem<T> for Chain<A, B> {
    type Tail = B::Tail;
    fn render_head(self, w: &mut T) -> Result<Self::Tail, fmt::Error> {
        let Chain { top, bottom } = self;
        top.render_all(w)?;
        bottom.render_head(w)
    }
}

pub struct Single<D, F> {
    tag: D,
    attr: F,
}

impl<T: fmt::Write, D: fmt::Display, F> RenderElem<T> for Single<D, F>
where
    F: FnOnce(&mut crate::AttrWriter<T>) -> std::fmt::Result,
{
    type Tail = ();
    fn render_head(self, w: &mut T) -> Result<Self::Tail, fmt::Error> {
        let Single { tag, attr } = self;
        crate::write_single(w, &tag, attr)?;
        Ok(())
    }
}

pub fn single<'a, D: fmt::Display + 'a, T: fmt::Write, F>(tag: D, attr: F) -> Single<D, F>
where
    F: FnOnce(&mut crate::AttrWriter<T>) -> std::fmt::Result + 'a,
{
    Single { tag: tag, attr }
}

pub fn elem<'a, D: fmt::Display + 'a, T: fmt::Write, F>(tag: D, attr: F) -> Elem<D, F>
where
    F: FnOnce(&mut crate::AttrWriter<T>) -> std::fmt::Result + 'a,
{
    Elem { tag: tag, attr }
}

pub struct ElemTail<D> {
    tag: D,
}

impl<D: fmt::Display, T: fmt::Write> RenderTail<T> for ElemTail<D> {
    fn render(self, w: &mut T) -> std::fmt::Result {
        crate::write_tail(w, &self.tag)
    }
}

pub struct Elem<D, F> {
    tag: D,
    attr: F,
}

impl<T: fmt::Write, D: fmt::Display, F> RenderElem<T> for Elem<D, F>
where
    F: FnOnce(&mut crate::AttrWriter<T>) -> std::fmt::Result,
{
    type Tail = ElemTail<D>;
    fn render_head(self, w: &mut T) -> Result<Self::Tail, fmt::Error> {
        let Elem { tag, attr } = self;
        crate::write_elem(w, &tag, attr)?;
        Ok(ElemTail { tag })
    }
}

#[test]
fn test_svg() {
    use crate::empty_attr;
    let potato = elem("potato", empty_attr);
    //let svg = elem("svg", empty_attr);
    let chicken = elem("chicken", empty_attr);
    let html = elem("html", empty_attr);
    let single = single("single", empty_attr);

    //let k=html.append(svg.append(potato).append(chicken));

    let k = html.append(potato.chain(chicken).chain(single));
    //let k=html.append(potato).append(chicken);
    //let html = elem("html", crate::empty_attr);

    let mut w = crate::upgrade_write(std::io::stdout());
    k.render_all(&mut w).unwrap();
    println!();
}
