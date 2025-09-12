// Calculate the factorial of a number using both iterative and recursive approaches.
fn factorial(n: u32) -> u64 {
    if n == 0 { return 1 };
    let mut result = 1;
    for i in 1..=n {
        result *= i as u64;
    }
    result
}

fn factorial_recursive(n: u32) -> u64 {
    if n == 0 { 1 } else { n as u64 * factorial_recursive(n - 1) }
}

fn main() {
  println!("{}", factorial(5));
  println!("{}", factorial_recursive(5));
}
