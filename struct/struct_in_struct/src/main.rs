#[derive(Debug)]
struct Address {
    street: String,
    city: String,
}

#[derive(Debug)]
struct Person {
    name: String,
    address: Address,
}

impl Address {
    fn new(street: String, city: String) -> Self {
        Self { street, city }
    }
}

impl Person {
    fn new(name: String, address: Address) -> Self {
        Self { name, address }
    }

    fn print_info(&self) {
        println!("Info:");
        println!("Name: {}", self.name);
        println!("Address: {} - {}", self.address.street, self.address.city);
    }

    fn move_to_city(&mut self, new_city: String) {
        self.address.city = new_city;
    }
}

fn main() {
    let mut p = Person::new(
        String::from("Thangquyeenf"),
        Address::new(String::from("Van Don"), String::from("Quang Ninh")),
    );
    p.print_info();
    p.move_to_city(String::from("Ha Noi"));
    p.print_info();
}
