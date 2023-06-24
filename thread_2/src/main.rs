use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    threads_with_shared_mutex();
}


fn threads_with_shared_mutex() {
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
