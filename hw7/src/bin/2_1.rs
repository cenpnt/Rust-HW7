fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut result: Vec<(f64,f64)> = Vec::new();

    let mut new_args = args.len();
    if args.len() % 2 != 0{
        new_args -= 1;
    }

    for i in 1..new_args {
        if args[i].parse::<f64>().is_ok() == false{
            return println!("Invalid input")
        }
        if i % 2 == 0{
            continue;
        }else{   
            if i+1 >= new_args{           
            break;
        }
        result.push((args[i].parse().unwrap(), args[i+1].parse().unwrap()));
        }  
    }  
    result.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap()); 
    println!("Ascending order by x: {:?}", result);

    result.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    println!("Descending order by x: {:?}", result);

    result.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap()); 
    println!("Ascending order by y: {:?}", result);

    result.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    println!("Descending order by y: {:?}", result);
}