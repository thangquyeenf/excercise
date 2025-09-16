#[derive(Debug)]
struct Expert<'a> {
    part: &'a str,
}

impl<'a> Expert<'a> {
    fn new(part: &'a str) -> Self {
        Self { part }
    }
}

fn main() {
    let part = "Thangquyeenf";
    let expert = Expert::new(&part);
    println!("{:?}", expert);
    {
        let scope_part = "into scope";
        let inner_expert = Expert::new(&scope_part);
        println!("inner_expert: {:?}", inner_expert);

    }
    // let outer_expert = Expert::new(&scope_part);
}
