use rand::{seq::SliceRandom};

fn main() {
    // 1 2 4 8
    // A O U K
    
    let letters = vec!['А', 'О', 'У', 'К'];
    let numbers = vec!['1', '2', '4', '8'];
    let c = letters.choose(&mut rand::thread_rng()).expect("Выбот из списка 'letters'");
    let n = numbers.choose(&mut rand::thread_rng()).unwrap();
    println!("{}{}", c, n);

    // собрать все возможные комбинации
    let mut all_combinations = Vec::new();
    for c in &letters {
        for n in &numbers {
            all_combinations.push(format!("{}{}", c, n));
        }
    }
    println!("all combinations = {:?}", all_combinations);
    println!("len = {:?}", all_combinations.len());
    all_combinations.shuffle(&mut rand::thread_rng());
    println!("shuffle = {:?}", all_combinations);
    

}
