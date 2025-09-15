fn main() {
    let s = String::from("hello");  // s comes into scope
    // let s1 = s;                 // s is moved to s1
    // println!("{}", s);        // this line would cause a compile-time error because s is no longer valid
    // println!("{}", s1);         // this works fine, s1 is valid
    let s_clone = s.clone(); // s is cloned to s_clone
    println!("s = {}, s_clone = {}", s, s_clone); // both s and s_clone are valid
}
