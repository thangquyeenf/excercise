#[derive(Debug, Clone)]

struct Pair<T> {
  first: T,
  second: T,
}

impl<T> Pair<T> {
  fn new(first: T, second: T) -> Self {
    Self { first, second }
  }

  fn longest_display<'a>(&'a self) -> &'a T 
  where T: std::fmt::Display + std::cmp::PartialOrd
  {
    if self.first >= self.second {
      &self.first
    } else {
      &self.second
    }
  }
}

impl<T: ToString> Pair<T> {
  fn longest<'a>(&'a self) -> &'a T {
    if self.first.to_string().len() >= self.second.to_string().len() {
      &self.first
    } else {
      &self.second
    }
  }
}




fn main() {
    let pair_i32 = Pair::<i32>::new(2, 5);
    println!("Pair i32: {:?}", pair_i32);
    println!("Longest: {}", pair_i32.longest_display());
    let pair_string = Pair::<String>::new(String::from("Thang"), String::from("Quyen"));
    println!("Pair String: {:?}", pair_string);
    println!("Longest: {}", pair_string.longest());
}
