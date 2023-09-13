use std::fs::File;
use std::io::Write;
fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Invalid")
    }

    for i in 1..args.len() {
        if args[i].parse::<f64>().is_ok() == false{
            println!("Invalid")
        }else{
            continue;
        }
    }

    let num: f64 = args[1].parse().unwrap();

    let mut table = String::new();
    table.push_str("<!DOCTYPE html>
    <html>
        <head>
            <title>Power</title>
            <h1>Power</h1>
            <style>table, th, td {border:1px solid black;text-align: center;width: 50%;border-collapse: collapse;table-layout: fixed;}</style>
        </head>
        <body>
        <table>
            <thead>
                <tr>
                    <th>x</th>
                    <th>x^2</th>
                    <th>x^3</th>
                </tr>
            </thead>
            <tbody>
            ");
    table.push_str("<tr>");
    table.push_str(&format!("<td>{}</td>", num));
    table.push_str(&format!("<td>{}</td>", num*num));
    table.push_str(&format!("<td>{}</td>", num*num*num));
    table.push_str("</tr>
                    </tbody>
                </table>
            </body>
        </html>
    ");

    let mut file = File::create("power.html").expect("none");
    file.write(table.as_bytes()).expect("none");

    println!("Created a file power.html")
}