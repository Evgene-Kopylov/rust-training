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

    // single_thread(urls[0].to_string());
    multi_thread(urls.to_vec());
}


fn multi_thread(urls: Vec<&str>) {
    let mut hands = Vec::new();
    for url in urls {
        let url = url.to_string();
        let hand = thread::spawn(move || {
            println!("multi_thread START");
            let response = reqwest::blocking::get(&url).unwrap();
            println!("{} {}", response.status(), &url);
            println!("multi_thread END");
            response
        });
        hands.push(hand);
    }

    for hand in hands {
        hand.join().unwrap();     
    }

}


/// Функция загрузки контента из одной ссылки в однопоточном режиме.
///
/// ## Аргументы
///
/// * `url` - ссылка для загрузки.
fn _single_thread(url: String) {
    // создание нового потока
    let hand = thread::spawn(move || {
        println!("single_thread START");
        let response = reqwest::blocking::get(&url).unwrap();
        println!("{} {}", response.status(), &url);
        println!("single_thread END");
        response
    });

    // ожидание завершения потока и получение результата
    let response = hand.join().unwrap();
    println!("status code: {:?}", response.status());

    // получение текста ответа
    let text = response.text().unwrap();

    // задание имени файла для сохранения ответа
    let file_name = "target/1.html".to_string();

    // запись ответа в файл
    write_all(text, file_name).unwrap();
}

/// Функция записи контента в файл.
///
/// ## Аргументы
///
/// * `content` - содержимое файла.
/// * `file_name` - имя файла для сохранения.
fn write_all(content: String, file_name: String) -> Result<(), io::Error> {
    // создание нового файла
    let mut file = File::create(file_name)?;

    // запись содержимого в файл
    file.write_all(content.as_bytes())?;

    Ok(())
}