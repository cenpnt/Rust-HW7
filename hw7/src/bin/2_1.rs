fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        return;
    }

    let mut check: Vec<f64> = Vec::new();

    for i in 1..args.len() {
        if args[i].parse::<f64>().is_ok() == false {
            println!("Invalid input");
            return;
        }
        check.push(args[i].parse().unwrap());
    }

    if check.len() % 2 != 0 {       
        check.pop();
    }

    let mut result: Vec<(f64, f64)> = Vec::new();

    for j in 0..check.len() / 2 {
        result.push((check[j * 2], check[j * 2 + 1]));
    }

    result.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap().then_with(|| a.1.partial_cmp(&b.1).unwrap()));
    println!("Ascending order: {:?}", result);

    result.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap().then_with(|| b.1.partial_cmp(&a.1).unwrap()));
    println!("Descending order: {:?}", result);
}
