use std::collections::HashMap;

fn char_count(s: &str) -> HashMap<char, usize> {
    let mut char_count = HashMap::new();
    for c in s.chars() {
        if c.is_whitespace() {
            continue;
        }
        *char_count.entry(c).or_insert(0) += 1;
    }
    char_count
}

fn main() {
    let s = "hello world";
    let counts = char_count(s);
    for (c, count) in counts {
        println!("'{}': {}", c, count);
    }
}
