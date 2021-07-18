use tagger::element;
use tagger::*;


fn main(){
    let mut k=PathBuilder::new();
    k.draw(PathCommand::L(5,3));
    k.draw_z();
    k.finish();

    let mut p=PointsBuilder::new();
    p.add(5,3);
    p.add(5.0,3.5);
    p.add(5,3);
    p.finish();


    let mut svg=element(move_format!("<svg {} {}>",p,k),"</svg>");

    //let mut svg=empty_elem!("svg");
    let mut g=empty_elem!("g");
    g.append(one_thing!("hello man"));
    g.append(one_thing!("hello man"));
    svg.append(g);
    svg.append(one_thing!("adfsadfs"));
    
    println!("{}",svg);
    
    println!("{}",svg);
}
