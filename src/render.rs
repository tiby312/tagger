
    pub trait Render {
        fn render(self, w: &mut dyn std::fmt::Write) -> std::fmt::Result;

        fn wrap<R:RenderBoth>(self,outer:R)->Wrap<Self,R> where Self:Sized{
            Wrap { inner: self, outer }
        }

    }

    impl<K:std::fmt::Display> Render for K{
        fn render(self, w: &mut dyn std::fmt::Write) -> std::fmt::Result{
            write!(w,"{}",self)
        }
    }

    pub struct Wrap<A,B>{
        inner:A,
        outer:B
    }

    impl<A,B> Render for Wrap<A,B> where A:Render,B:RenderBoth{
        fn render(self, w: &mut dyn std::fmt::Write) -> std::fmt::Result{
            let (res,second)=self.outer.render_both(w);
            res?;
            self.inner.render(w)?;
            second.render(w)
        }
    }

    pub trait RenderBoth {
        type Next: Render;
        fn render_both(self, w: &mut dyn std::fmt::Write) -> (std::fmt::Result, Self::Next);
    }

    pub struct Single<F>(F);
    impl<F> Render for Single<F>
    where
        F: FnOnce(&mut dyn std::fmt::Write) -> std::fmt::Result,
    {
        fn render(self, w: &mut dyn std::fmt::Write) -> std::fmt::Result {
            self.0(w)
        }
    }

    pub struct Pair<A, B> {
        first: A,
        second: B,
    }
    impl<A, B> Pair<A, B>
    where
        A: FnOnce(&mut dyn std::fmt::Write) -> std::fmt::Result,
        B: FnOnce(&mut dyn std::fmt::Write) -> std::fmt::Result,
    {   
        pub fn new(first:A,second:B)->Self{
            Pair{
                first,
                second
            }
        }

    }


    pub struct AsSingle<K>(K);
    impl<K:RenderBoth> Render for AsSingle<K>{
        fn render(self, w: &mut dyn std::fmt::Write) -> std::fmt::Result {
            let (res,next)=self.0.render_both(w);
            res?;
            next.render(w)
        }

    }

    impl<A, B> RenderBoth for Pair<A, B>
    where
        A: FnOnce(&mut dyn std::fmt::Write) -> std::fmt::Result,
        B: FnOnce(&mut dyn std::fmt::Write) -> std::fmt::Result,
    {
        type Next = Single<B>;
        fn render_both(self, w: &mut dyn std::fmt::Write) -> (std::fmt::Result, Self::Next) {
            let res = (self.first)(w);
            (res, Single(self.second))
        }
    }


    pub fn elem()->impl RenderBoth{
        Pair::new(|w|write!(w,"<svg>"),|w|write!(w,"</svg>"))
    }

    #[test]
    fn test_svg(){
        let svg=elem();
        let k="hello";

        let k=k.wrap(svg).wrap(elem());

        let mut w = crate::upgrade_write(std::io::stdout());
        k.render(&mut w).unwrap();
        println!();
    }

