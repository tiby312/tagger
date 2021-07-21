use tagger::*;

fn main() {
    let width = 500.0;
    let height = 400.0;

    let mut svg = {
        let svg_attr = new_attr()
            .attr("xmlns", "http://www.w3.org/2000/svg")
            .attr("viewBox", formatm!("0 0 {} {}", width, height))
            .finish();

        elem!("svg", svg_attr)
    };

    let path = {
        use PathCommand::*;
        let path = new_path()
            .add(M(200, 120))
            .add(Q(300, 50, 400, 120))
            .add(T(500, 120))
            .finish();

        let gc = new_attr()
            .attr("stroke", "black")
            .attr("stroke-width", 2)
            .attr("fill", "green")
            .attr("fill-opacity", 0.5)
            .attr_whole(path)
            .finish();

        elem!("path", gc)
    };

    svg.append(path);

    let path = {
        use PathCommand::*;
        let path = new_path()
            .add(M(300, 200))
            .add(H_(-150))
            .add(A_(150, 150, 0, 1, 0, 150, -150))
            .add(Z(""))
            .finish();

        let gc = new_attr()
            .attr("stroke", "black")
            .attr("stroke-width", 2)
            .attr("fill", "blue")
            .attr("fill-opacity", 0.5)
            .attr_whole(path)
            .finish();

        elem!("path", gc)
    };

    svg.append(path);

    println!("{}", svg);
}
