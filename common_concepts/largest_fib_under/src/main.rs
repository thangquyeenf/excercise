fn factorial(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn largest_fib_under(limit: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;
    while b < limit {
        let temp = b;
        b = a + b;
        a = temp;
    }
    a
}

fn main() {
    println!("factorial of 5 is {}", factorial(5));
    println!("{}", largest_fib_under(100));
}
