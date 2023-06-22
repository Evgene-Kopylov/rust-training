use std::{fs};
use reqwest::Error;
use std::io::prelude::*;
use std::thread;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let urls = [
        "https://www.google.com/",
        "https://www.github.com/",
        "https://www.wikipedia.org/",
        "https://www.youtube.com/",
        "https://www.stackoverflow.com/",
    ];
    
    single_thread(urls[0].to_owned())?;

    Ok(())
}


fn single_thread(url: String) -> Result<(), Error> {
    let hand = thread::spawn(move || {
        println!("single_thread Перед");
        let response = reqwest::blocking::get(&url).unwrap();
        println!("{} {}", response.status(), &url);
        println!("single_thread После");
        response
    });
    
    let response = hand.join().unwrap();
    Ok(())
}


async fn download_url(url: &str) -> Result<String, Error> {
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
