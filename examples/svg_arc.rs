use tagger::*;

fn main() {
    let width = 500.0;
    let height = 400.0;

    let mut svg = {
        let svg_attr = AttrBuilder::new()
            .attr("xmlns", "http://www.w3.org/2000/svg")
            .attr("viewBox", move_format!("0 0 {} {}", width, height))
            .finish();

        element(move_format!("<svg {}>", svg_attr), "</svg>")
    };

    let path = {
        use PathCommand::*;
        let path = PathBuilder::new()
            .add(M(200, 120))
            .add(Q(300, 50, 400, 120))
            .add(T(500, 120))
            .finish();

        let gc = AttrBuilder::new()
            .attr("stroke", "black")
            .attr("stroke-width", 2)
            .attr("fill", "green")
            .attr("fill-opacity", 0.5)
            .attr_whole(path)
            .finish();

        elem_single!(move_format!("<path {}/>", gc))
    };

    svg.append(path);

    let path = {
        use PathCommand::*;
        let path = PathBuilder::new()
            .add(M(300, 200))
            .add(H_(-150))
            .add(A_(150, 150, 0, 1, 0, 150, -150))
            .add_z()
            .finish();

        let gc = AttrBuilder::new()
            .attr("stroke", "black")
            .attr("stroke-width", 2)
            .attr("fill", "blue")
            .attr("fill-opacity", 0.5)
            .attr_whole(path)
            .finish();

        elem_single!(move_format!("<path {}/>", gc))
    };

    svg.append(path);

    println!("{}", svg);
}
