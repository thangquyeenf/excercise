use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for i in 0..10 {
        let counter_clone = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
            println!("Thread {}: Counter is {}", i, *num);
            thread::sleep(Duration::from_millis(50));
        }));
    }
    handles
        .into_iter()
        .for_each(|handle| handle.join().unwrap());
    println!("Final counter value: {}", *counter.lock().unwrap());
}
