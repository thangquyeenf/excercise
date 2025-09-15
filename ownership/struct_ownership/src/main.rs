
// #[derive(Debug)]
// struct User {
//     name: String,
//     age: u32,
// }

// fn main() {
//     let user1 = User {
//         name: String::from("Alice"),
//         age: 30,
//     };
//     let username = &user1.name;
//     println!("Username: {}", username);
//     println!("User1: {:?}", user1);
// }


#[derive(Debug)]
struct User<'a> {
    name: &'a str,
    age: u32,
}

fn main() {
    let name = String::from("Alice");
    let user1 = User {
        name: &name,
        age: 30,
    };
    let username = user1.name;
    println!("Username: {}", username);
    println!("User1: {:?}", user1);
}