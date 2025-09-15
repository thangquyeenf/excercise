fn main() {
    let mut s = String::from("hello world");
    let s_str = "hello world";
    print_slice(&s);
    print_slice(s_str);
    let s_slice = &s[0..5];
    s.push_str("!!!");
    print_slice(s_slice);
}

fn print_slice(s: &str) {
    println!("{}", s);
}