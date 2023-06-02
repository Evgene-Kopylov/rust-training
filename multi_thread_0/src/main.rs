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
    
    for i in 0..urls.len() {
        let url = urls[i];
        let content = download_url(url).await?;
        let file_name = format!("{i}.html");

        // запись в файл
        match write_all(content, file_name.clone()) {
            Err(e) => println!("{:?}", e),
            _ => ()
        }
        println!("{}", file_name);
    }

    Ok(())
}


async fn download_url(url: &str) -> Result<String, reqwest::Error> {
    let content = reqwest::get(url)
        .await?
        .text()
        .await?;
    Ok(content)
}

fn write_all(content: String, file_name: String) -> std::io::Result<()> {
    let mut file = fs::File::create(file_name)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
