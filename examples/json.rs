use tagger::prelude::*;

struct Sentinal;

fn main() -> core::fmt::Result {
    let mut io = tagger::upgrade(std::io::stdout());

    let mut root = tagger::json(&mut io)?;

    root.inner("chicken", "pizza")?;
    let mut p = root.elem("potato")?;
    p.inner("flop", "flap")?;
    p.inner("ddd", 5)?;
    p.inner("dflap", 42)?;

    let mut k = p.elem("table")?;
    k.inner("kay", 4)?;
    k.inner("fadf", 3)?;
    k.end()?;
    p.end()?;

    root.end()?;

    Ok(())
}
