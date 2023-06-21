// use reqwest;


// fn main() {
//     let urls = [
//         "https://www.google.com/",
//         "https://www.github.com/",
//         "https://www.wikipedia.org/",
//         "https://www.youtube.com/",
//         "https://www.stackoverflow.com/"
//     ];

//     for i in 0..urls.len() {
//         let url = urls[i];
//         let mut req = reqwest::blocking::get(url).unwrap();
//         println!("{:?}", url);
//     }
// }


use std::error::Error;
use tokio::task;
use reqwest::Url;

#[tokio::main]
async fn main() {
    println!("-=-=-=--=- Отдельные варианты -=-=-=-=-");
    single_task().await.unwrap();
}

async fn single_task() -> Result<(), Box<dyn Error>> {
    println!("single_task Before sleep");
    let url = Url::parse("https://www.google.com").unwrap();
    let _response = reqwest::get(url).await?;    
    println!("single_task After sleep");
    Ok(())
}
