use reqwest;


fn main() {
    let urls = [
        "https://www.google.com/",
        "https://www.github.com/",
        "https://www.wikipedia.org/",
        "https://www.youtube.com/",
        "https://www.stackoverflow.com/"
    ];

    for i in 0..urls.len() {
        let url = urls[i];
        let mut req = reqwest::blocking::get(url).unwrap();
        println!("{:?}", url);
    }
}
