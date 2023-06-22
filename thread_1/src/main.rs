use std::{thread, time};

fn main() {
    let urls = [
        "https://www.google.com/",
        "https://www.github.com/",
        "https://www.wikipedia.org/",
        "https://www.youtube.com/",
        "https://www.stackoverflow.com/",
    ];

    single_thread(urls[0].to_string());
}

fn single_thread(url: String) {
    let hand = thread::spawn(move || {
        println!("single_thread START");
        let response = reqwest::blocking::get(&url).unwrap();
        println!("{} {}", response.status(), &url);
        println!("single_thread END");
        response.status()
    });

    let val = hand.join().unwrap();
    println!("val: {:?}", val);
}
