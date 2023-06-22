use std::{thread, time};

fn main() {
    single_thread();
}

fn single_thread() {
    let hand = thread::spawn(move || {
        println!("single_thread START");
        thread::sleep(time::Duration::from_secs(2));
        println!("single_thread END");
        100
    });

    let val = hand.join().unwrap();
    println!("val: {:?}", val);
}