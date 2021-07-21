use tagger::*;

fn main() {
    let mut root = single!("<!DOCTYPE html>");

    let style = elem!("style").add(single!(
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

    root.append(style);

    let table = {
        let f = new_attr().attr("style", formatm!("width:{}%", 100)).build();

        let mut table = elem!("table", f);

        for i in 0..20 {
            let mut tr = elem!("tr");

            tr.append(elem!("th").add(single!(formatm!("Hay {}:1", i))));

            tr.append(elem!("th").add(single!(formatm!("Hay {}:2", i))));

            tr.append(elem!("th").add(single!(formatm!("Hay {}:3", i))));

            table.append(tr);
        }
        table
    };

    root.append(table);

    println!("{}", root);
}
