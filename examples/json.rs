use tagger::prelude::*;

fn main() -> core::fmt::Result {
    let mut io = tagger::upgrade(std::io::stdout());

    let mut root = tagger::elem(&mut io, wrstr!("{"), wrstr!("}"))?;

    let mut gloss=root.elem(wrstr!(r#"menu": {"#),wrstr!("}"))?;
    gloss.single(wr!(r#""id":"file","#))?;
    gloss.single(wr!(r#"value:file,"#))?;
    let mut p=gloss.elem(wrstr!(r#""popup":{"#),wrstr!("}"))?;
    p.single(wrstr!(r#""hay":"foo""#))?;

    p.end()?;
    gloss.end()?;

    root.end()?;
    Ok(())
}
