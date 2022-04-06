use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let bin_url = &args[1];
    let mut num = 0;
    loop {
        let resp = reqwest::get(bin_url).await?;
        num += 1;
        println!("[ {} ] | Status - {}", num, resp.status());
    }
}