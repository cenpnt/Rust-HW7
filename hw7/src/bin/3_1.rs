use std::fs::File;
use std::io::Write;
fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Invalid input");
    }

    for i in 1..args.len() {
        if args[i].parse::<f64>().is_ok() == false{
            println!("Invalid")
        } else{
            continue;
        }        
    }
    let a1: f64 = args[1].parse().unwrap();
    let a2: f64 = args[2].parse().unwrap();
    let a3: f64 = args[3].parse().unwrap();    
    
    if a3 < 0.0{
        return;
    }

    let mut table = String::new();
    table.push_str("<!DOCTYPE html>
    <html>
        <head>
            <title>Fahr To Celcius</title>
            <h1>Fahr to Cel</h1>
            <style>table, th, td {border:1px solid black;text-align: center;width: 50%;border-collapse: collapse;}</style>
        </head>
        <body>
        <table>
            <thead>
                <tr>
                    <th>Fahr</th>
                    <th>Celcius</th>
                </tr>
            </thead>
            <tbody>
            ");

    let mut i = a1;
        if a1 == 0. && a2 == 0. && a3 == 0. {                       
           table.push_str("<tr><td>0.0</td><td>0.0</td></tr>") ;
        }else{            
            if a1 < a2{
                while i <= a2 {
                let c = (5./9.) * (i - 32.0);
                let c = format!("{:.1}", c);            
                table.push_str(&format!("<tr><td>{}</td><td>{}</td></tr>", i, c));    
                i += a3;            
                   }
            }else{
                while i >= a2 {
                let c = (5./9.) * (i - 32.0);
                let c = format!("{:.1}", c);           
                table.push_str(&format!("<tr><td>{}</td><td>{}</td></tr>", i, c)); 
                i -= a3;
                }
            }
        }

        table.push_str("</tbody>
                            </table>
                        </body>
                    </html>
                    ");

        let mut file = File::create("fahr_to_cel.html").expect("none");
        file.write(table.as_bytes()).expect("none");

        println!("Created a file fahr_to_cel.html")
    }
