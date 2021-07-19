use tagger::element;
use tagger::*;

fn main() {
    let path = PathBuilder::new()
        .draw(PathCommand::L(5, 3))
        .draw_z()
        .finish();
    let points = PointsBuilder::new().add(5, 3).add(5.0, 3.5).finish();

    let attr = AttrBuilder::new()
        .attr("width", 5)
        .attr("height", 6)
        .attr_whole(path)
        .finish();

    let mut svg = elem_single!(move_format!("<svg {} {}/>", points, attr));

    let mut g = empty_elem!("g");
    g.append(elem_single!("hello man"));
    g.append(elem_single!("hello man"));
    svg.append(g);
    svg.append(elem_single!("adfsadfs"));

    println!("{}", svg);

    println!("{}", svg);
}
