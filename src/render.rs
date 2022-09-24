use std::fmt;
pub trait Render<T: fmt::Write> {
    fn render(self, w: &mut T) -> std::fmt::Result;

    fn wrap<R: RenderBoth<T>>(self, outer: R) -> Wrap<Self, R>
    where
        Self: Sized,
    {
        Wrap { inner: self, outer }
    }
}

impl<K: std::fmt::Display, T: fmt::Write> Render<T> for K {
    fn render(self, w: &mut T) -> std::fmt::Result {
        write!(w, "{}", self)
    }
}

pub struct Wrap<A, B> {
    inner: A,
    outer: B,
}

impl<A, B, T: fmt::Write> Render<T> for Wrap<A, B>
where
    A: Render<T>,
    B: RenderBoth<T>,
{
    fn render(self, w: &mut T) -> std::fmt::Result {
        let (res, second) = self.outer.render_both(w);
        res?;
        self.inner.render(w)?;
        second.render(w)
    }
}

pub trait RenderBoth<T: fmt::Write> {
    type Next: Render<T>;
    fn render_both(self, w: &mut T) -> (std::fmt::Result, Self::Next);
}

pub struct Single<F>(F);
impl<F, T: fmt::Write> Render<T> for Single<F>
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

pub struct AsSingle<K>(K);
impl<K: RenderBoth<T>, T: fmt::Write> Render<T> for AsSingle<K> {
    fn render(self, w: &mut T) -> std::fmt::Result {
        let (res, next) = self.0.render_both(w);
        res?;
        next.render(w)
    }
}

impl<A, B, T: fmt::Write> RenderBoth<T> for Pair<A, B>
where
    A: FnOnce(&mut T) -> std::fmt::Result,
    B: FnOnce(&mut T) -> std::fmt::Result,
{
    type Next = Single<B>;
    fn render_both(self, w: &mut T) -> (std::fmt::Result, Self::Next) {
        let res = (self.first)(w);
        (res, Single(self.second))
    }
}

pub fn elem<'a, T: fmt::Write>(
    tag: &'a str,
    func: impl FnOnce(&mut crate::AttrWriter<T>) -> std::fmt::Result + 'a,
) -> impl RenderBoth<T> + 'a {
    Pair::new(
        move |w: &mut T| crate::write_elem(w, &tag, func),
        move |w: &mut T| write!(w, "</{}>", tag),
    )
}

#[test]
fn test_svg() {
    let svg = elem("svg", crate::empty_attr);
    let k = "hello";

    let k = k.wrap(svg).wrap(elem("svg", |w| w.attr("pizza", 5)));

    let mut w = crate::upgrade_write(std::io::stdout());
    k.render(&mut w).unwrap();
    println!();
}
