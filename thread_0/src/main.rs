use std::{thread, time};


fn main() {
    // single_thread();
    multy_threads();
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

fn multy_threads() {
    let mut hands = Vec::new();
    for i in 0..10 {
        let hand = thread::spawn(move || {
            println!("multy_threads {i} START");
            thread::sleep(time::Duration::from_secs(2));
            println!("multy_threads {i} END");
        });
        hands.push(hand);
    }

    for hand in hands {
        hand.join().unwrap();
    }
}
