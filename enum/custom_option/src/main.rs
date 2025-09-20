
#[derive(Debug)]
enum MyOption<T> {
    Some(T),
    None,
}

impl<T> MyOption<T> {
    fn map<U, F>(self, f: F) -> MyOption<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            MyOption::Some(v) => MyOption::Some(f(v)),
            MyOption::None => MyOption::None,
        }
    }

    fn unwrap_or(self, default: T) -> T {
        match self {
            MyOption::Some(v) => v,
            MyOption::None => default,
        }
    }
}


fn main() {
    let x = MyOption::Some(5);
    let y = x.map(|v| v * 2); // Should be MyOption::Some(10)
    println!("y = {:?}", y);

    let z: MyOption<i32> = MyOption::None;
    let value = z.unwrap_or(42); // Should be 42
    println!("value = {}", value);
}