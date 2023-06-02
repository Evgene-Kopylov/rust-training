#![allow(unused)]

use rand::{seq::SliceRandom};

fn main() {
    // 1 2 4 8
    // A O U K
    
    let letters = vec!['А', 'О', 'У', 'К'];
    let numbers = vec!['1', '2', '4', '8'];
    let c = letters.choose(&mut rand::thread_rng()).unwrap();
    let n = numbers.choose(&mut rand::thread_rng()).unwrap();
    // println!("{}{}", c, n);

    // собрать все возможные комбинации
    let mut all_combinations = Vec::new();
    for c in &letters {
        for n in &numbers {
            all_combinations.push((c, n));
        }
    }
    // println!("all combinations = {:?}", all_combinations);
    // println!("len = {:?}", all_combinations.len());
    all_combinations.shuffle(&mut rand::thread_rng());
    // println!("shuffle = {:?}", all_combinations);
    
    // сложить пасьянс
    let mut res = all_combinations.clone();
    for _ in 0..all_combinations.len() {
        for i in 2..res.len() {
            if res[i].0 == res[i-2].0
            || res[i].1 == res[i-2].1 {
                // println!("{:?}", res[i]);
                res.remove(i);
                break;
            }
        }
    }
    // print!("сложенно {:?}", res);

    // собрать комбинацию
    let mut collection = Vec::new();
    let step = 1;
    for i in 0..step {
        collection.push(all_combinations.pop().unwrap());
    }
    println!("collection = {:?}", collection);
    for _ in step..all_combinations.len() {
        for i in 0..collection.len() {
            if all_combinations[i].0 == collection[collection.len() - step].0
            || all_combinations[i].1 == collection[collection.len() - step].1 {
                collection.push(all_combinations[i]);
                all_combinations.remove(i);
                break;
            }
        }
    }
    println!("collection = {:?}", collection);
    print!("{}", collection.len());

}
