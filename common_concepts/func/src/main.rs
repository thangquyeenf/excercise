/*
1. Viết một hàm square(n: i32) -> i32 trả về bình phương của n.
Create a function, that return the square of n: square(n: i32) -> 32

2. Viết một hàm greet(name: &str) in ra "Hello, {name}!".
Create a function greet(name: &str) that prints "Hello, {name}!"

3. Viết hàm fahrenheit_to_celsius(f: f64) -> f64 để chuyển đổi nhiệt độ.
Create a function fahrenheit_to_celsius(f: f64) -> f64, to convert temperature
*/


// 1.
fn square(n: i32) -> i32 {
  n * n
}

// 2.
fn greet(name: &str) {
  println!("Hello, {name}!");
}

// 3.
fn fahrenheit_to_celsius(f: f64) -> f64 {
  (f - 32.0) / 1.8
}

fn main() {
    println!("Square of 5 is: {}", square(5));
    let name = "Thangquyeenf";
    greet(name);
    println!("123.0F to C: {}", fahrenheit_to_celsius(123.0))
}
