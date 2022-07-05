fn main() -> std::fmt::Result {
    let mut first = String::new();
    let mut second = String::new();
    let mut w = tagger::new(&mut first as &mut dyn std::fmt::Write);

    w.elem("foo", tagger::no_attr())?.build(|w| {
        w.elem("bar", tagger::no_attr())?.build(|w| {
            w.swap_writer(&mut second as &mut dyn std::fmt::Write);
            Ok(())
        })
    })?;

    println!("{}<TEST>{}", first, second);

    Ok(())
}
