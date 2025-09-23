use std::sync::{ Arc, Mutex };
use std::thread;
use std::time::Duration;


fn main() {
    let a = Arc::new(Mutex::new(5));
    let mut handles = vec![];
    for i in 0..5 {
        let a_clone = Arc::clone(&a);
        handles.push(thread::spawn(move || {
            let mut num = a_clone.lock().unwrap();
            *num += 1;
            println!("Thread {}: Updated value to {}", i, *num);
            thread::sleep(Duration::from_millis(50));
        }));
    }
    // for handle in handles {
    //     handle.join().unwrap();
    // }
    handles.into_iter().for_each(|handle| handle.join().unwrap());
    println!("Main: Value is {}", *a.lock().unwrap());
}
