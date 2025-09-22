use std::env;

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    let result = divide(10.0, 2.0);
    match result {
        Ok(value) => println!("Result: {}", value),
        Err(e) => println!("Error: {}", e),
    }

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a number as a command line argument.");
        return;
    }

    let input = &args[1];

    match input.trim().parse::<i32>() {
        Ok(num) => println!("Num: {}", num),
        Err(e) => println!("Error: {}", e),
    }
}
