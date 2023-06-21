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
    few_tasks().await;
}

async fn single_task() -> Result<(), Box<dyn Error>> {
    println!("single_task Before sleep");
    let url = Url::parse("https://www.google.com").unwrap();
    let _response = reqwest::get(url).await?;    
    println!("single_task After sleep");
    Ok(())
}


async fn few_tasks() {
    let task1 = async {
        println!("few_tasks 1 START");
        let url = Url::parse("https://www.github.com/").unwrap();
        let response = reqwest::get(url).await.unwrap();    
        println!("few_tasks 1 FIN");
        response
    };

    let task2 = async {
        println!("few_tasks 2 START");
        let url = Url::parse("https://www.wikipedia.org/").unwrap();
        let response = reqwest::get(url).await.unwrap();    
        println!("few_tasks 2 FIN");
        response
    };

    let (res1, res2) = tokio::join!(task1, task2);

    println!("{:?},  {:?}", res1.status(), res2.status());
}
