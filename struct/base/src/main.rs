#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn new(width: u32, height: u32) -> Self {
    Self { width, height }
  }

  fn area(&self) -> u32 {
    self.width*self.height
  }

  fn can_hold(&self, other: &Rectangle) -> bool {
    self.area() > other.area()
  }
}

fn main() {
    let rec = Rectangle::new(4, 5);
    println!("Area: {}", rec.area());
    let bigger_rec = Rectangle::new(10, 6);
    println!("bigger_rec can hold: {}", bigger_rec.can_hold(&rec));
}
