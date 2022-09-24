use std::fmt;
pub trait RenderTail<T: fmt::Write> {
    fn render(self, w: &mut T) -> std::fmt::Result;
}

impl<T:fmt::Write> RenderTail<T> for (){
    fn render(self, _: &mut T) -> std::fmt::Result{
        Ok(())
    }
}


pub trait RenderBoth<T: fmt::Write> {
    type Tail: RenderTail<T>;
    fn render_both(self, w: &mut T) -> Result<Self::Tail, fmt::Error>;

    fn append<R: RenderBoth<T>>(self, bottom: R) -> Append<Self, R>
    where
        Self: Sized,
    {
        Append { top: self, bottom }
    }

    fn wrap<R: RenderBoth<T>>(self, outer: R) -> Wrap<Self, R>
    where
        Self: Sized,
    {
        Wrap { inner: self, outer }
    }
}


pub struct Wrap<A, B> {
    inner: A,
    outer: B,
}

impl<A, B, T: fmt::Write> RenderBoth<T> for Wrap<A, B>
where
    A: RenderBoth<T>,
    B: RenderBoth<T>,
{
    type Tail = TailChain<A::Tail, B::Tail>;
    fn render_both(self, w: &mut T) -> Result<Self::Tail, fmt::Error> {
        let tail_outer = self.outer.render_both(w)?;
        let tail_inner = self.inner.render_both(w)?;

        Ok(TailChain {
            a: tail_inner,
            b: tail_outer,
        })
    }
}

pub struct TailChain<A, B> {
    a: A,
    b: B,
}
impl<A: RenderTail<T>, B: RenderTail<T>, T: fmt::Write> RenderTail<T> for TailChain<A, B> {
    fn render(self, w: &mut T) -> std::fmt::Result {
        self.a.render(w)?;
        self.b.render(w)
    }
}

fn render_all<R: RenderBoth<T>, T: fmt::Write>(a: R, w: &mut T) -> fmt::Result {
    let next = a.render_both(w)?;
    next.render(w)
}

pub struct Append<A, B> {
    top: A,
    bottom: B,
}

impl<A: RenderBoth<T>, B: RenderBoth<T>, T: fmt::Write> RenderBoth<T> for Append<A, B> {
    type Tail = B::Tail;
    fn render_both(self, w: &mut T) -> Result<Self::Tail, fmt::Error> {
        let Append { top, bottom } = self;
        render_all(top, w)?;
        bottom.render_both(w)
    }
}

pub struct ClosureTail<F>(F);
impl<F, T: fmt::Write> RenderTail<T> for ClosureTail<F>
where
    F: FnOnce(&mut T) -> std::fmt::Result,
{
    fn render(self, w: &mut T) -> std::fmt::Result {
        self.0(w)
    }
}

pub struct Pair<A, B> {
    first: A,
    second: B,
}

impl<A, B> Pair<A, B> {
    pub fn new(first: A, second: B) -> Self {
        Pair { first, second }
    }
}

impl<A, B, T: fmt::Write> RenderBoth<T> for Pair<A, B>
where
    A: FnOnce(&mut T) -> std::fmt::Result,
    B: FnOnce(&mut T) -> std::fmt::Result,
{
    type Tail = ClosureTail<B>;
    fn render_both(self, w: &mut T) -> Result<Self::Tail, fmt::Error> {
        (self.first)(w)?;
        Ok(ClosureTail(self.second))
    }
}

pub fn elem<'a, T: fmt::Write>(
    tag: &'a str,
    func: impl FnOnce(&mut crate::AttrWriter<T>) -> std::fmt::Result + 'a,
) -> impl RenderBoth<T> + 'a {
    Pair::new(
        move |w: &mut T| crate::write_elem(w, &tag, func),
        move |w: &mut T| crate::write_tail(w, &tag),
    )
}

#[test]
fn test_svg() {
    let potato = elem("potato", |w| w.attr("id", 5));
    let svg = elem("svg", crate::empty_attr);
    let html = elem("html", crate::empty_attr);

    let k = potato.wrap(svg).append(html);

    let mut w = crate::upgrade_write(std::io::stdout());
    render_all(k, &mut w).unwrap();
    println!();
}
