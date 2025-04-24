use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // TODO: Create a shared counter using Arc and Mutex
    let mutex = Mutex::new(0);
    let val = Arc::new(mutex);

    // TODO: Create a vector to store thread handles
    let mut handles = vec![];
    
    // TODO: Spawn 5 threads
    for _ in 1..=5 {
        // TODO: Clone the Arc for the thread
        let new_val = val.clone();
        
        // TODO: Spawn a thread that increments the counter 10 times
        let handle = thread::spawn(move || {
            // TODO: Increment counter 10 times
            for _ in 0..10 {
                *new_val.lock().unwrap() += 1;
            }
            
        });
        
        handles.push(handle);
    }
    
    // TODO: Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap()
    }
    
    // TODO: Print the final value of the counter
    println!("Final result {}", *val.lock().unwrap());
}