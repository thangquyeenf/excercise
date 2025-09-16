
#[derive(Debug)]
struct Point {
  x: f64,
  y: f64,
}

impl Point {
  fn new(x: f64, y: f64) -> Point {
    Point {x, y}
  }

  fn distance(&self, other: &Point) -> f64 {
    ((other.x - self.x).powi(2) + (other.y - self.y).powi(2)).sqrt()
  }
}

fn main() {
    let p = Point::new(1.0, 2.0);
    println!("{:?}", p);
    let o_p = Point::new(4.3, 2.3);
    println!("distance {:?}", p.distance(&o_p));
}
