use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Logger {
    messages: RefCell<Vec<String>>,
}

impl Logger {
    fn new() -> Self {
        Logger {
            messages: RefCell::new(vec![]),
        }
    }

    fn log(&self, msg: &str) {
        self.messages.borrow_mut().push(msg.to_string());
    }

    fn show_logs(&self) {
        for msg in self.messages.borrow().iter() {
            println!("{}", msg);
        }
    }
}

fn main() {
    let logger = Rc::new(Logger::new());
    let l1 = Rc::clone(&logger);
    let l2 = Rc::clone(&logger);
    logger.log("First log entry");
    logger.log("Second log entry");
    l1.log("Third log entry from l1");
    l2.log("Fourth log entry from l2");
    logger.show_logs();
}
