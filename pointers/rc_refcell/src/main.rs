use rc_refcell::List;
use rc_refcell::Node;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let node_1 = Rc::new(RefCell::new(Node::new(1, None)));
    let node_2 = Rc::new(RefCell::new(Node::new(2, None)));
    let node_3 = Rc::new(RefCell::new(Node::new(3, None)));

    node_1.borrow_mut().set_next(Some(Rc::clone(&node_2)));
    node_2.borrow_mut().set_next(Some(Rc::clone(&node_3)));

    println!("{:?}", node_1.borrow());
    println!("Node 1 value: {}", node_1.borrow().get_value());
    if let Some(next) = node_1.borrow().get_next() {
        next.borrow_mut().set_value(20);
    }
    node_1.borrow().display();

    let a = Rc::new(List::Cons(5, RefCell::new(Rc::new(List::Nil))));
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(List::Cons(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count after b created = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after making cycle = {}", Rc::strong_count(&b));
    println!("a rc count after making cycle = {}", Rc::strong_count(&a));

    // println!("a next item = {:?}", a.tail());
}
