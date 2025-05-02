use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;

// Define a special value that will signal termination
const TERMINATION_SIGNAL: i32 = -1;

fn main() {
    // Number of items to produce
    const ITEM_COUNT: usize = 20;
    
    // TODO: Create a channel for sending numbers
    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));

    let mut produce = vec![];
    let mut consume = vec![];
    
    // TODO: Create 2 producer threads
    let num_producers = 2;
    for i in 0..num_producers {
        let tx_clone = tx.clone();
        let handle = thread::spawn(move || {
            producer(i, tx_clone, ITEM_COUNT);
        });
        produce.push(handle);
    }
    
    // TODO: Create 3 consumer threads
    let num_consumers = 3;
    for i in 0..num_consumers {
        let rx_clone = Arc::clone(&rx);
        let handle = thread::spawn(move || {
            consumer(i, rx_clone);
        });

        consume.push(handle);
    }
    
    // TODO: Wait for all threads to finish
    for p in produce { p.join().unwrap() }
    for _ in 0..num_consumers { tx.send(TERMINATION_SIGNAL).unwrap(); }
    for c in consume { c.join().unwrap(); }
    
    println!("All items have been produced and consumed!");
}

// TODO: Implement producer function
fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize) {
    // TODO: Generate random numbers and send them to the channel
    let mut rng = rand::thread_rng();
    for _ in 0..item_count {
        let num = rng.gen_range(1..100);
        println!("Producer {} sending: {}", id, num);
        tx.send(num).unwrap();
        thread::sleep(Duration::from_millis(100));
    }
    // When finished, producer should NOT send termination signal
    println!("Producer {} has finsihed producing", id)
}

// TODO: Implement consumer function
fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    // TODO: Receive numbers from the channel and process them
    loop {
        let num = rx.lock().unwrap().recv().unwrap();
        // Break the loop when receiving the termination signal
        match num {
            TERMINATION_SIGNAL => {
                println!("Consumer {} has been terminated", id);
                break;
            },
            _ => {
                println!("Consumer {} consumes {}", id, num);
            }
        }
        thread::sleep(Duration::from_millis(100));
    }
    

}
