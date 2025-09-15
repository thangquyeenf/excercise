fn main() {
    println!("Hello, world!");
}

fn word_count(s: &str) -> usize {
    s.split_whitespace().count()
}

fn word_count_plain(s: &str) -> usize {
    let mut count = 0;
    let mut in_word = false;

    for c in s.chars() {
        if c.is_whitespace() {
            if in_word {
                in_word = false;
            }
        } else {
            if !in_word {
                in_word = true;
                count += 1;
            }
        }
    }

    count
}

