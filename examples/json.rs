fn main() -> core::fmt::Result {

    tagger::json::json(&mut tagger::upgrade(std::io::stdout()))?.defer_end(|root|{

    
        root.inner("chicken", "pizza")?;
        root.elem("potato")?.defer_end(|p|{
            p.inner("flop", "flap")?;
            p.inner("ddd", 5)?;
            p.inner("dflap", 42)?;
        
            p.elem("table")?.defer_end(|k|{
                k.inner("kay", 4)?;
                k.inner("fadf", 3)
            })
            
        })
    })
}
