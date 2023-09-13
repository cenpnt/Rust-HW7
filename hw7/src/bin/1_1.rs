fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2{
         println!("Invalid input");
    }
    let mut result: Vec<i64> = Vec::new();  
 
     for i in 1..args.len() {
         if args[i].parse::<i64>().is_ok() == false {
             return println!("Invalid input");
         }
         result.push(args[i].parse().unwrap());
     }
 
     result.sort_by(|a, b| a.partial_cmp(b).unwrap()); 
     println!("Ascending order: {:?}", result);
 
     result.sort_by(|a, b| b.partial_cmp(a).unwrap());
     println!("Descending order: {:?}", result);
 
 }