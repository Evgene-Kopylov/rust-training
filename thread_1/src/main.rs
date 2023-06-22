use std::fs::File;
use std::io;
use std::io::Write;
use std::thread;

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

/// Выполнение задачи в единственном потоке. Используя `std::thread`
/// ## Аргументы
/// `url` - адрес страници запроса
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
    write_all(text, file_name).unwrap();
}

/// Запись контента в файл
/// ## Аргументы 
/// * `content` - текст
/// * `file_name` - имя файла, включая относительный путь
fn write_all(content: String, file_name: String) -> Result<(), io::Error> {
    let mut file = File::create(file_name)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
