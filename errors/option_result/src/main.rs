use std::collections::HashMap;

struct Users {
    users: HashMap<u32, String>,
}

impl Users {
    fn new() -> Self {
        Users {
            users: HashMap::new(),
        }
    }

    fn add_user(&mut self, id: u32, name: String) {
        self.users.insert(id, name);
    }

    fn get_user(&self, id: u32) -> Option<&String> {
        self.users.get(&id)
    }

    fn find_username(&self, id: u32) -> Option<&String> {
        self.get_user(id)
    }

    fn get_username(&self, id: u32) -> Result<&String, String> {
        match self.get_user(id) {
            Some(name) => Ok(name),
            None => Err(format!("User with ID {} not found", id)),
        }
    }
}

fn main() {
    let mut users = Users::new();
    users.add_user(1, "Alice".to_string());
    users.add_user(2, "Bob".to_string());
    users.add_user(3, "Charlie".to_string());
    let user_id = 2;
    match users.find_username(user_id) {
        Some(name) => println!("User found: {}", name),
        None => println!("User with ID {} not found", user_id),
    }
    match users.get_username(user_id) {
        Ok(name) => println!("User found: {}", name),
        Err(e) => println!("Error: {}", e),
    }
}
