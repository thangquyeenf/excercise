use std::io;
use std::io::Write;

fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn reverse_string_from_sratch(s: &str) -> String {
    let mut reversed = String::new();
    for c in s.chars() {
        reversed.insert(0, c);
    }
    reversed
}

fn count_vowels(s: &str) -> usize {
    s.chars()
        .filter(|&c| "aeiouAEIOU".contains(c))
        .count()
}


fn print_string_with_space(s: &str) {
    let words: Vec<&str> = s.split_whitespace().collect();
    for word in words {
        println!("{}", word);
    }
}

fn main() {
    let entry = io::stdout();
    let mut handle = entry.lock();
    writeln!(handle, "Enter a string: ").unwrap();
    handle.flush().unwrap();

    let mut input = String::new();
    if let Err(e) = io::stdin().read_line(&mut input) {
        eprintln!("Đọc input lỗi: {}", e);
        return;
    }

    print_string_with_space(&input.trim());
    println!("Reversed (using built-in): {}", reverse_string(&input.trim()));
    println!("Reversed (from scratch): {}", reverse_string_from_sratch(&input.trim()));
    println!("Number of vowels: {}", count_vowels(&input.trim()));
}
