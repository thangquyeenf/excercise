fn main() {
    let s = String::from("hello");
    let word = first_word(&s);
    println!("First word is: {}", word);
    let word2 = first_word_v2(&s);
    println!("First word v2 is: {}", word2);
    let first_word_idx = first_word_v3(&s);
    println!("First word idx is: {}", first_word_idx);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word_v2(s: &str) -> &str {
  let i = s.find(' ').unwrap_or(s.len());
  &s[..i]
}

fn first_word_v3(s: &str) -> usize {
  let i = s.find(' ').unwrap_or(s.len());
  i
}

fn fisrt_word_v4(s: &str) -> &str {
  if let Some(i) = s.find(' ') {
    &s[..i]
  } else {
    s
  }
}

