use std::sync::Arc;
use std::thread;

fn main() {
    let numbers = Arc::new(vec![1, 2, 3, 4, 5]);
    let mut handles = vec![];
    for i in 0..5 {
        let numbers_clone = Arc::clone(&numbers);
        handles.push(thread::spawn(move || {
            let sum: i32 = numbers_clone.iter().sum();
            println!("Thread {}: Sum of {:?} is {}", i, *numbers_clone, sum);
        }));
    }
    handles
        .into_iter()
        .for_each(|handle| handle.join().unwrap());
    println!("Main: Numbers are {:?}", *numbers);
}
