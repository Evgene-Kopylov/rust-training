use std::time::Duration;
use tokio::time::sleep;
use tokio::task;

#[tokio::main]
async fn main() {
    println!("-=-=-=--=- Отдельные варианты -=-=-=-=-");
    single_task().await;
    few_tasks().await;
    many_tasks().await;

    println!("-=-=-=--=- Все вместе -=-=-=-=-");
    tokio::join!(single_task(), few_tasks(), many_tasks());
}

async fn single_task() {
    println!("single_task Before sleep");
    sleep(Duration::from_secs(2)).await;
    println!("single_task After sleep");
}

async fn few_tasks() {
    let task1 = async {
        println!("task1 Before sleep");
        sleep(Duration::from_secs(2)).await;
        println!("task1 After sleep");
    };

    let task2 = async {
        println!("task2 Before sleep");
        sleep(Duration::from_secs(1)).await;
        println!("task2 After sleep");
    };

    let (_res1, _res2) = tokio::join!(task1, task2);

    println!("{:?},  {:?}", _res1, _res2);
}

async fn many_tasks() {
    let tasks = vec![
        task::spawn(async {
            println!("many_tasks 1. START");
            sleep(Duration::from_secs(2)).await;
            println!("many_tasks 1. FINISH");
        }),
        task::spawn(async {
            println!("many_tasks 2. START");
            sleep(Duration::from_secs(2)).await;
            println!("many_tasks 2. FINISH");
        }),
        task::spawn(async {
            println!("many_tasks 3. START");
            sleep(Duration::from_secs(2)).await;
            println!("many_tasks 3. FINISH");
        })];
    
    for task in tasks {
        task.await.unwrap();
    }


}
