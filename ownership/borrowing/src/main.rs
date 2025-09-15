
fn print_length(s: &String) {
    println!("The length of the string is: {}", s.len());
}

fn append_word(s: &mut String) {
    s.push_str(" World!");
}

fn main() {
    let my_string = String::from("Hello, Rust!");
    print_length(&my_string);

    let mut my_mut_string = String::from("Hello");
    append_word(&mut my_mut_string);
    println!("{}", my_mut_string);

    let mut x = 5;
    {
        let y = &mut x;
        println!("y: {}", y);
    }
    let z = &mut x;
    println!("z: {}", z);
}
