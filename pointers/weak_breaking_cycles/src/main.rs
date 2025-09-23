use std::rc::{ Rc, Weak};
use std::cell::RefCell;
// use std::sync::{ Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>,
}

impl Node {
    fn new(value: i32) -> Rc<Node> {
        Rc::new(Node {
            value,
            children: RefCell::new(vec![]),
            parent: RefCell::new(Weak::new()),
        })
    }

    fn add_child(parent: &Rc<Node>, child: Rc<Node>) {
        parent.children.borrow_mut().push(Rc::clone(&child));
        *child.parent.borrow_mut() = Rc::downgrade(parent);
    }

    fn display(&self, level: usize) {
        for _ in 0..level {
            print!("  ");
        }
        println!("|--{}", self.value);
        for child in self.children.borrow().iter() {
            child.display(level + 1);
        }
    }

    fn parent(&self) -> Option<Rc<Node>> {
        self.parent.borrow().upgrade()
    }
}
  

fn main() {
    let root = Node::new(1);
    let child_1 = Node::new(2);
    let child_2 = Node::new(3);
    let grandchild_1 = Node::new(4);
    let grandchild_2 = Node::new(5);

    Node::add_child(&root, Rc::clone(&child_1));
    Node::add_child(&root, Rc::clone(&child_2));
    Node::add_child(&child_1, Rc::clone(&grandchild_1));
    Node::add_child(&child_1, Rc::clone(&grandchild_2));

    root.display(0);

    println!("Reference count of root: {}", Rc::strong_count(&root));
    println!("Reference count of child_1: {}", Rc::strong_count(&child_1));
    println!("Reference count of grandchild_1: {}", Rc::strong_count(&grandchild_1));

    // weak reference count
    println!("Weak reference count of root: {}", Rc::weak_count(&root));
    println!("Weak reference count of child_1: {}", Rc::weak_count(&child_1));
    println!("Weak reference count of grandchild_1: {}", Rc::weak_count(&grandchild_1));

    if let Some(parent) = child_1.parent() {
        println!("Parent of child_1: {}", parent.value);
    } else {
        println!("child_1 has no parent");
    }

    if let Some(parent) = root.parent() {
        println!("Parent of root: {}", parent.value);
    } else {
        println!("root has no parent");
    }

}
