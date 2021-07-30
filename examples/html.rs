use tagger::prelude::*;

fn main() {
    let mut root = tagger::Element::one_new("<!DOCTYPE html>");

    root.append(elem!("style").appendm(
        "table, th, td {
            border: 1px solid black;
            border-collapse: collapse;
            animation: mymove 5s infinite;
          }
          @keyframes mymove {
              from {background-color: red;}
              to {background-color: blue;}
          }",
    ));

    root.append({
        let mut table = elem!("table", ("style", formatm!("width:{}%", 100)));

        for i in 0..20 {
            let mut tr = elem!("tr");

            tr.append(elem!("th").appendm(formatm!("Hay {}:1", i)));

            tr.append(elem!("th").appendm(formatm!("Hay {}:2", i)));

            tr.append(elem!("th").appendm(formatm!("Hay {}:3", i)));

            table.append(tr);
        }
        table
    });

    println!("{}", root.display());
}
