use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // threads_with_shared_mutex();
    threads_with_channal()
}

fn threads_with_channal() {
    let (tx, rx) = mpsc::channel();

    for i in 0..10 {
        let tx_i = tx.clone();
        thread::spawn(move || {
            tx_i.send(i).unwrap();
        });
    }

    for received in rx {
        println!("{}", received);
    }
}

fn _threads_with_shared_mutex() {
    let target = vec![];
    let counter = Arc::new(Mutex::new(target));
    let mut handles = vec![];

    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut val = counter.lock().unwrap();

            val.push(i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {:?}", *counter.lock().unwrap());
}
