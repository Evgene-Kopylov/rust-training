use std::{fs};
use reqwest;
use reqwest::Error;
use std::io::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // чтение содержимого файла в строку
    let content = fs::read_to_string("urls.txt")
        .expect("Failed to read");

    // разбить строку по символу переноса
    let urls: Vec<&str> = content.lines().collect();
    println!("{:?}", urls);

    let url = urls[0];
    let content = reqwest::get(url)
        .await?
        .text()
        .await?;
    let file_name = "1.html".to_string();

    // запись в файл
    match write_all(content, file_name) {
        Err(e) => println!("{:?}", e),
        _ => ()
    }
    
    Ok(())
}


fn write_all(content: String, file_name: String) -> std::io::Result<()> {
    let mut file = fs::File::create(file_name)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
