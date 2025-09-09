/*
1. Viết chương trình nhập một số n. Nếu n chẵn thì in "Even", nếu lẻ thì in "Odd".

2. Viết hàm max(a: i32, b: i32) -> i32 trả về số lớn hơn.

3. Viết chương trình in ra:

  "Fizz" nếu chia hết cho 3

  "Buzz" nếu chia hết cho 5

  "FizzBuzz" nếu chia hết cho cả 3 và 5

  Ngược lại in ra chính số đó.

*/

// 1.
fn isEven(n: i32) {
  if n % 2 == 0 {
    println!("Even")
  } else {
    println!("Odd")
  } 
}


// 2.
fn max(a: i32, b: i32) -> i32 {
  if a > b { a } else { b }
}

// 3.
fn issue_three(n: i32) {
  if n % 3 == 0 && n % 5 == 0 {
    println!("FizzBuzz");
  } else if n % 5 == 0 {
    println!("Buzz");
  } else if n % 3 == 0 {
    println!("Fizz");
  } else {
    println!("{}", n);
  }
}

fn main() {
    isEven(3);
    println!("{}", max(52,6));
    issue_three(13);
}
