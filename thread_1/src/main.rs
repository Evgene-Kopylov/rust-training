use std::io::Write;
use std::thread;
use std::fs::File;

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
        response
    });

    let response = hand.join().unwrap();
    println!("status code: {:?}", response.status());
    let text = response.text().unwrap();
    println!("text len: {:?}", text.len());
    let file_name = "target/1.html".to_string();
    write_all(text, file_name);
}

fn write_all(content: String, file_name: String) {
    let mut file = File::create(file_name).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}
