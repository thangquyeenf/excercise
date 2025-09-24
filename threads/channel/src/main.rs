use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
    let mut handles = vec![];
    for i in 0..3 {
        let tx_clone = tx.clone();
        handles.push(thread::spawn(move || {
            for j in 0..5 {
              let msg = format!("Hello {} from thread {}",j, i);
              tx_clone.send(msg).unwrap();
            }
        }));
    }

    for _handle in handles {
        _handle.join().unwrap();
    }

    drop(tx); // Close the original sender to avoid deadlock
    for received in rx {
        println!("Received: {}", received);
    }
}
    