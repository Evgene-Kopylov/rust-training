use std::{fs, io::Write};
use reqwest;
use reqwest::Error;
// use std::io::Error as io_error;


#[tokio::main]
async fn main() -> Result<(), Error> {
    // чтение содержимого файла в строку
    let content = fs::read_to_string("urls.txt")
        .expect("Failed to read");
    // println!("{}", content); 

    // разбить строку по символу переноса
    let urls: Vec<&str> = content.lines().collect();
    println!("{:?}", urls);

    let url = urls[0];
    let content = reqwest::get(url)
        .await?
        .text()
        .await?;
    println!("{:?}", content);
    let path = url.replace('.', "_");
    // let mut file = fs::File::create(&path)?;
    // file.write_all(buf)
    Ok(())
}