#[derive(Debug, Clone)]
struct Pair<T> {
  first: T,
  second: T,
}

impl<T: Clone> Pair<T> {
  fn new(first: T, second: T) -> Self {
    Self { first, second }
  }

  fn swap(&mut self) {
    let tmp = self.first.clone();
    self.first = self.second.clone();
    self.second = tmp;
  }

  fn swap_v2(&mut self) {
    std::mem::swap(&mut self.first, &mut self.second);
  }
}

fn main() {
    let mut pair_i32 = Pair::<i32>::new(2, 5);
    println!("Before: {:?}", pair_i32);
    pair_i32.swap();
    println!("After: {:?}", pair_i32);

    let mut pair_string = Pair::<String>::new(String::from("Thang"), String::from("Quyen"));
    println!("Before: {:?}", pair_string);
    pair_string.swap_v2();
    println!("After: {:?}", pair_string);

}
