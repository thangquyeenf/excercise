use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
  value: i32,
  next: Option<Rc<RefCell<Node>>>,
}

impl Node {
  fn new(value: i32, next: Option<Rc<RefCell<Node>>>) -> Self {
    Self { value, next }
  }
}

fn main() {
    let node_1 = Rc::new(RefCell::new(Node::new( 1, None )));
    let node_2 = Rc::new(RefCell::new(Node::new( 2, None )));
    let node_3 = Rc::new(RefCell::new(Node::new( 3, None )));

    node_1.borrow_mut().next = Some(Rc::clone(&node_2));
    node_2.borrow_mut().next = Some(Rc::clone(&node_3));
    
    println!("{:?}", node_1);

    let another_node = Rc::new(RefCell::new(Node::new(100, Some(Rc::clone(&node_3)))));
    println!("another node: {:?}", another_node);
}
