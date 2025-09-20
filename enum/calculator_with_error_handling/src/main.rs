enum Operation {
    Add(i32, i32),
    Sub(i32, i32),
    Div(i32, i32),
}

fn calculate(op: Operation) -> Result<i32, String> {
  match op {
    Operation::Add(a, b) => Result::Ok(a + b),
    Operation::Sub(a, b) => Result::Ok(a - b),
    Operation::Div(a, b ) => {
      if b == 0 {
        Result::Err("Division by zero".to_string())
      } else {
        Result::Ok(a / b)
      }
    }
  }
}

fn main() {
    let ops = [
        Operation::Add(2, 3),
        Operation::Sub(10, 4),
        Operation::Div(8, 2),
        Operation::Div(5, 0),
    ];

    for op in ops {
        match calculate(op) {
            Ok(result) => println!("Result: {}", result),
            Err(e) => println!("Error: {}", e),
        }
    }
}
