/*
1. Viết chương trình tính tổng từ 1 đến 100 bằng:

  1.1 loop

  1.2 while

  1.3 for

2. Viết chương trình in ra dãy Fibonacci 10 số đầu tiên.

3. Viết chương trình đảo ngược một chuỗi "hello" → "olleh" bằng vòng lặp.
*/

// 1.1
fn sum_using_loop() -> i32 {
  let mut sum = 0;
  let mut iter = 0;
  loop {
    iter += 1;
    if iter > 100 {
      break;
    }
    sum += iter;
  }
  sum
} 

// 1.2
fn sum_using_while() -> i32 {
  let mut sum = 0;
  let mut iter = 0;
  while iter < 100 {
    iter += 1;
    sum += iter;
  }
  sum
}

// 1.3
fn sum_using_for() -> i32 {
  let mut sum = 0;
  for i in 0..100 {
    sum += i+1;
  }
  sum
}

fn main() {
    println!("{}", sum_using_loop());
    println!("{}", sum_using_while());
    println!("{}", sum_using_for());
}
