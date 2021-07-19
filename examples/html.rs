use tagger::*;

fn main() {
    let mut root = elem_single!("<!DOCTYPE html>");

    let style = {
        let mut style = element("<style>", "</style>");
        style.append(elem_single!(
            "table, th, td {
            border: 1px solid black;
            border-collapse: collapse;
            animation: mymove 5s infinite;
          }
          @keyframes mymove {
              from {background-color: red;}
              to {background-color: blue;}
          }"
        ));
        style
    };

    root.append(style);

    let table = {
        let f = AttrBuilder::new()
            .attr("style", move_format!("width:{}%", 100))
            .finish();
        let mut table = element(move_format!("<table {}>", f), "</table>");

        for i in 0..20 {
            let mut tr = element("<tr>", "</tr>");

            let th =
                element("<th>", "</th>").append_move(elem_single!(move_format!("Hay {}:1", i)));
            tr.append(th);

            let th =
                element("<th>", "</th>").append_move(elem_single!(move_format!("Hay {}:2", i)));
            tr.append(th);

            let th =
                element("<th>", "</th>").append_move(elem_single!(move_format!("Hay {}:3", i)));
            tr.append(th);

            table.append(tr);
        }
        table
    };

    root.append(table);

    println!("{}", root);
}
