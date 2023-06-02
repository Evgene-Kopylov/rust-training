// use std::thread;

fn main() {
    let handle = std::thread::spawn(move || {
        println!("Hello, world!");
    });
    // handle.join();

}
