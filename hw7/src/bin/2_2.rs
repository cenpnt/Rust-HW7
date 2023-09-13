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

    bubble_sort_x(& mut result);
    println!("Ascending order by x: {:?}", result);
    
    bubble_sort_x_des(& mut result);
    println!("Descending order by x: {:?}", result);

    bubble_sort_y(& mut result);
    println!("Ascending order by y: {:?}", result);
    
    bubble_sort_y_des(& mut result);
    println!("Descending order by y: {:?}", result);
}

fn bubble_sort_x(list: &mut Vec<(f64,f64)>) {
    let num = list.len();
    for i in 0..num - 1{
        for j in 0..num - i - 1{
            if list[j].0 > list[j + 1].0 {
                let temp = list[j].0;
                list[j].0 = list[j + 1].0;
                list[j + 1].0 = temp;
            }
        }
    }
}

fn bubble_sort_x_des(list: &mut Vec<(f64,f64)>) {
    let num = list.len();
    for i in 0..num - 1{
        for j in 0..num - i - 1{
            if list[j].0 < list[j + 1].0 {
                let temp = list[j].0;
                list[j].0 = list[j + 1].0;
                list[j + 1].0 = temp;
            }
        }
    }
}

fn bubble_sort_y(list: &mut Vec<(f64,f64)>) {
    let num = list.len();
    for i in 0..num - 1{
        for j in 0..num - i - 1{
            if list[j].1 > list[j + 1].1 {
                let temp = list[j].1;
                list[j].1 = list[j + 1].1;
                list[j + 1].1 = temp;
            }
        }
    }
}

fn bubble_sort_y_des(list: &mut Vec<(f64,f64)>) {
    let num = list.len();
    for i in 0..num - 1{
        for j in 0..num - i - 1{
            if list[j].1 < list[j + 1].1 {
                let temp = list[j].1;
                list[j].1 = list[j + 1].1;
                list[j + 1].1 = temp;
            }
        }
    }
}