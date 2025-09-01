use std::env::args;
use calculator;


fn main() {
    let args: Vec<String> = args().skip(1).collect();

    let a = match args.get(0){
        Some(val) => match val.parse::<f64>() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Please use a number");
                return;
            }
        },
        None => { 
            eprintln!("First nuber not  given");
            return;
        }
    };

    let b = match args.get(2) {
        Some(val) => match val.parse::<f64>() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Please use a number");
                return;
            }
        },
        None => {
            eprintln!("Second number not given");
            return;
        }
    };


    let op = match args.get(1) {
        Some(val) => val.as_str(),
        None => {
            eprintln!("No symbol given");
            return;
        }
    };

    let result = match op {
        "+" => calculator::add_num(a, b),
        "-" => calculator::subtract_num(a, b),
        "/" => calculator::divide_num(a, b).expect("Division failed"),
        "*" => calculator::multiply_num(a, b),
        _ => {
            eprintln!("Please use correct symbol (+, -, /, *)");
            return;
        }
    };

    println!("{} {} {} = {}", a, op, b, result);


}
