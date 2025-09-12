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

// 2.
fn fibonacci(n: usize) -> Vec<u32> {
  if n == 0 { return vec![]; }
  if n == 1 { return vec![0]; }
  let mut fib = vec![0, 1];
  for i in 2..n {
    let next = fib[i-1] + fib[i-2];
    fib.push(next);
  }
  fib
}

// 3.
fn reverse_string(s: &str) -> String {
  let mut reversed = String::new();
  for c in s.chars().rev() {
    reversed.push(c);
  }
  reversed
}

fn main() {
    println!("{}", sum_using_loop());
    println!("{}", sum_using_while());
    println!("{}", sum_using_for());
    println!("{:?}", fibonacci(10));
    println!("{}", reverse_string("hello"));
}
