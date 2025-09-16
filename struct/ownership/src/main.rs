use rand::Rng;

#[derive(Debug)]
struct User {
    id: u32,
    username: String,
    email: String,
    active: bool,
}

impl User {
    fn new(id: u32, username: String, email: String, active: bool) -> Self {
        Self {
            id,
            username,
            email,
            active,
        }
    }

    fn create_user(username: String, email: String) -> User {
        let mut rng = rand::thread_rng();

        let id: u32 = rng.gen_range(1..=1000);
        User::new(id, username, email, true)
    }

    fn deactive(&mut self) {
        self.active = false;
    }
}

fn main() {
    let mut user = User::create_user(
        String::from("Thangquyeenf"),
        String::from("thangquyeenf@gmail.com"),
    );
    println!("Before: {:?}", user);
    user.deactive();
    println!("After: {:?}", user);
}
