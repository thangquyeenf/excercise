#[derive(Debug)]
struct User {
  id: u32,
  name: String,
}

impl User {
  fn new(id: u32, name: String) -> Self {
    Self { id, name }
  }
}

fn create_users() -> Vec<User> {
  vec![
    User::new(1, String::from("Diep")),
    User::new(2, String::from("Quyen")),
    User::new(3, String::from("Thang")),
  ]
}

fn find_user_by_name<'a>(users: &'a [User], name: &'a str) -> Option<&'a User> {
  for user in users {
    if user.name == name {
      return Some(user);
    }
  }
  None
}

fn find_user_by_name_v2<'b>(users: &'b [User], name: &str) -> Option<&'b User> {
  users.iter().find(|u| u.name == name)
}

fn remove_user_by_id(users: &mut Vec<User>, id: u32) {
  if let Some(pos) = users.iter().position(|u| u.id == id) {
    users.remove(pos);
  } else {
    println!("Invalid id");
  }
}

fn main() {
    let user = User::new(1, String::from("Thangquyeenf"));
    println!("{:?}", user);
    let mut users = create_users();
    println!("{:?}", users);
    if let Some(thang) = find_user_by_name(&users, &String::from("Thang")) {
      println!("{:?}", thang);
    }

    match find_user_by_name_v2(&users, &String::from("Rust")) {
      Some(u) => println!("{:?}", u),
      None => println!("User not found!"),
    }

    remove_user_by_id(&mut users, 32);
}
