fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u32) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn all_primes_up_to(n: u32) -> Vec<u32> {
    let mut primes = Vec::new();
    for i in 2..=n {
        if is_prime(i) {
            primes.push(i);
        }
    }
    primes
}

fn main() {
    println!("{}", is_prime(29));
    println!("{}", is_prime(25));
    println!("{}", (26 as f64).sqrt() as u32);
    println!("{:?}", all_primes_up_to(100));
}
