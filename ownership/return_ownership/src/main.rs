fn main() {
    let s = String::from("hello");
    takse_ownership(s);
    let s1 = give_ownership();
    let s2 = take_and_give_back(s1);
    println!("{}", s2);
}

fn takse_ownership(s: String) {
    println!("{}", s);
}

fn give_ownership() -> String {
    let s = String::from("hello");
    s
}

fn take_and_give_back(s: String) -> String {
    println!("{}", s);
    s
}