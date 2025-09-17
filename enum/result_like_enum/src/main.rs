#[derive(Debug)]
enum MyResult<T, E> {
    Ok(T),
    Err(E),
}

fn divide(a: i32, b: i32) -> MyResult<i32, String> {
    if b == 0 {
        MyResult::Err("Division by zero".to_string())
    } else {
        MyResult::Ok(a / b)
    }
}

fn main() {
    match divide(10, 2) {
        MyResult::Ok(val) => println!("Result: {}", val),
        MyResult::Err(e) => println!("Error: {}", e),
    }

    match divide(10, 0) {
        MyResult::Ok(val) => println!("Result: {}", val),
        MyResult::Err(e) => println!("Error: {}", e),
    }
}
