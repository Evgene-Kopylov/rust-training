use reqwest;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let response = reqwest::get("https://www.rust-lang.org").await?;

    println!("Status: {}", response.status());

    let body = response.text().await?;
    println!("Body:\n{}", body);


    Ok(())
}