fn is_palindrome(s: &str) -> bool {
    let cleaned: String = s.chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_lowercase().next().unwrap())
        .collect();
    cleaned == cleaned.chars().rev().collect::<String>()
}

fn main() {
    println!("{}", is_palindrome("Level"));
    println!("{}", is_palindrome("Level2"));
}
