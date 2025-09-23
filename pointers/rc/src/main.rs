use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
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
    let a = Rc::new(List::Cons(5, Rc::new(List::Cons(10, Rc::new(List::Nil)))));
    let b = List::Cons(3, Rc::clone(&a));
    let c = List::Cons(4, Rc::clone(&a));
    print_list(&a);
    print_list(&b);
    print_list(&c);
    println!("Reference count after creating b and c: {}", Rc::strong_count(&a));
    drop(c);
    println!("Reference count after dropping c: {}", Rc::strong_count(&a));
}
