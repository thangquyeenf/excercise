use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Node {
    pub value: i32,
    pub next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(value: i32, next: Option<Rc<RefCell<Node>>>) -> Self {
        Self { value, next }
    }

    pub fn set_next(&mut self, next: Option<Rc<RefCell<Node>>>) {
        self.next = next;
    }

    pub fn get_next(&self) -> Option<Rc<RefCell<Node>>> {
        self.next.as_ref().map(Rc::clone)
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }

    pub fn set_value(&mut self, value: i32) {
        self.value = value;
    }

    pub fn display(&self) {
        print!("{}", self.value);
        if let Some(next) = &self.next {
            print!(" -> ");
            next.borrow().display();
        } else {
            println!();
        }
    }
}
