use std::fmt::Write;
use tagger::xml2::AttrTrait;

static NORMAL:[&'static str;2]=["<","/>"];
static COMMENT:[&'static str;2]=["<!--","-->"];
static XML_DECL:[&'static str;2]=["<?","?>"];


fn main() -> core::fmt::Result {
    let mut io = tagger::upgrade(std::io::stdout());

    let mut root = tagger::xml2::Element::new(io);

    root.single("xml",XML_DECL,|a|{a.attr("version",1.0)?.attr("encoding","UTF-8")?.attr("standalone","no")})?;

    root.single("",COMMENT,|mut a|{write!(a,"{}","This is a comment")?;Ok(a)})?;

    let mut v:Vec<[f32;2]>=Vec::new();

    root.elem2("svg",move |elem_builder|{
        let svg=elem_builder.build(|attr_builder|{
            attr_builder.attr("chicken","but")?.polyline_data(|p|{
                for a in v.iter(){
                    p.add_point(*a)?;
                }
                Ok(p)
            })
        })?;

        println!("{:?}",v);

        svg.elem2("div",|builder|{
            let div=builder.build(|a|a.attr("chicken","potato"))?;
            div.single("rect",NORMAL,|a|a.attr("x1",5)?.attr("y1",6))?;
            div.ok()
        })?;
        


        svg.ok()
    })

}
