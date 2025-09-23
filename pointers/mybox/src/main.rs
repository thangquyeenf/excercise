use std::ops::Deref;
use std::fmt::Display;

#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Display> Display for MyBox<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}


impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Dropping MyBox");
    }
}

fn main() {
   let mb = MyBox::new(String::from("Hello, MyBox!"));
   println!("MyBox contains: {}", *mb);
   println!("MyBox contains (using Display): {}", mb);
    // MyBox will be dropped automatically here
}
