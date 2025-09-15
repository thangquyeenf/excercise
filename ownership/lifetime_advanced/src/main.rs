fn main() {
    let s1 = "Hello";
    {
      let s2  = "Word";
    }
    let longest = longest(&s1, &s2);
    println!("{}", longest);
}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
  if s1.len() > s2.len() { s1 } else { s2 }
}