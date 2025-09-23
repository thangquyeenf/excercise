
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn print_list(list: &List) {
    match list {
        List::Cons(value, next) => {
            print!("{} ", value);
            print_list(next);
        }
        List::Nil => {
            println!();
        }
    }
}

fn main() {
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));
    print_list(&list);
}
