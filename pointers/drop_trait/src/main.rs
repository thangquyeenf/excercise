
#[derive(Debug)]
struct CustomSmartPointer {
    data: String,
}

impl CustomSmartPointer {
    fn new(data: String) -> Self {
        Self { data }
    }

    fn display(&self) {
        println!("CustomSmartPointer data: {}", self.data);
    }
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data: {}", self.data);
    }
}

fn main() {
    let csp1 = CustomSmartPointer::new(String::from("First"));
    
    csp1.display();
    {
      let csp2 = CustomSmartPointer::new(String::from("Second"));
      csp2.display();
    }

    println!("End of main function");
}
