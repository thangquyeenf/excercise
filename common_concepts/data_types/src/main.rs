/*
1. Create a tuple containing (i32, f64, char) and print each element using destructuring.

2. Create an array numbers = [1, 2, 3, 4, 5]. Let's:

  2.1. Access the 3rd element

  2.2. Print the length of the array

  2.3. Create a slice from element 2 to 4
*/

fn main() {

  // 1. 
  let tup: (i32, f64, char) = (12, 2.5, 'c');
  let (i, f, c) = tup;

  println!("int: {}", i);
  println!("float: {}", f);
  println!("char: {}", c);

  // 2. 
  let arr = [1, 2, 3, 4, 5];
  // 2.1
  let _3rd_el = arr[2];
  println!("Access the 3rd element: {}", _3rd_el);
  // 2.2
  println!("length of array: {}", arr.len());
  // 2.3
  let a_slice = &arr[2..5];
  println!("Create a slice from element 2 to 4: {a_slice:?}");
}
