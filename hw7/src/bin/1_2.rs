fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2{
        println!("Invalid input");
    }

    let mut result: Vec<i64> = Vec::new();
    
    for i in 1..args.len() {
        if args[i].parse::<i64>().is_ok() == false{
            return println!("Invalid input")
        }
        result.push(args[i].parse().unwrap())
    }

    bubble_sort(& mut result);
    println!("Ascending order: {:?}", result);
    
    bubble_sort_des(& mut result);
    println!("Descending order: {:?}", result);
}


fn bubble_sort(list: &mut Vec<i64>) {
    let num = list.len();
    for i in 0..num - 1{
        for j in 0..num - i - 1{
            if list[j] > list[j + 1] {
                let temp = list[j];
                list[j] = list[j + 1];
                list[j + 1] = temp;
            }
        }
    }
}

fn bubble_sort_des(list: &mut Vec<i64>) {
    let num = list.len();
    for i in 0..num - 1{
        for j in 0..num - i - 1{
            if list[j] < list[j + 1] {
                let temp = list[j];
                list[j] = list[j + 1];
                list[j + 1] = temp;
            }
        }
    }
}