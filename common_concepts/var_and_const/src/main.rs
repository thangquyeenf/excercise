/* 
1. Declare a variable x with value 5. Then change the value of x to 10 by shadowing (without using mut).

2. Declare a constant PI with value 3.14159 and print it to the screen.
*/

fn main() {
    // 1. var
    let x = 5;
    println!("x: {}", x);
    let x = x + 5;
    println!("x after use shadowing: {}", x);

    // 2. const
    const PI: f64 = 3.14159;
    println!("Constant PI: {}", PI);
}
