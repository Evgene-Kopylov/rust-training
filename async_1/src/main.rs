use reqwest::Url;
use std::error::Error;
use tokio::task;

#[tokio::main]
async fn main() {
    // println!("-=-=-=--=- Отдельные варианты -=-=-=-=-");
    // single_task().await.unwrap();
    // few_tasks().await;
    // from_list().await;
    println!("-=-=-=--=- Все вместе -=-=-=-=-");
    tokio::join!(single_task(), few_tasks(), from_list());
}

async fn single_task() -> Result<(), Box<dyn Error>> {
    println!("single_task Before");
    let url = Url::parse("https://www.google.com").unwrap();
    let _response = reqwest::get(url).await?;
    println!("single_task After");
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

async fn from_list() {
    let urls = [
        "https://www.google.com/",
        "https://www.github.com/",
        "https://www.wikipedia.org/",
        "https://www.youtube.com/",
        "https://www.stackoverflow.com/",
    ];

    let mut tasks = Vec::new();
    for line in urls {
        let t: task::JoinHandle<reqwest::Response> = task::spawn(async move {
            println!("from_list {} START", line);
            let url = Url::parse(line).unwrap();
            let response = reqwest::get(url).await.unwrap();
            println!("from_list {} FIN {}", line, response.status());
            response
        });
        tasks.push(t);
    }
    for task in tasks {
        task.await.unwrap();
    }
}
