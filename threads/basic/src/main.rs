use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("Hello from a spawned thread!");
    });
    println!("Hello from the main thread!");
    handle.join().unwrap();

    let numbers = vec![1, 2, 3, 4, 5];
    let handle = thread::spawn(move || {
        let sum: i32 = numbers.iter().sum();
        println!("Sum of {:?} is {}", numbers, sum);
    });
    println!("Waiting for the thread to finish...");
    handle.join().unwrap();
}
