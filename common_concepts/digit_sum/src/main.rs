fn digit_sum(n: u32) -> u32 {
    let mut sum = 0;
    let mut num = n;
    while num > 0 {
        sum += num % 10;
        num /= 10;
    }
    sum
}

fn main() {
    println!("number: 12345, digit sum: {}", digit_sum(12345));
}
