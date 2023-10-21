// Healh check monitoring
// @author Ipan Ardian

use dotenv::dotenv;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let url = std::env::var("HEALTHCHECK_URL").expect("HEALTHCHECK_URL must be set");
    let client = Client::new();

    match client.get(url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                println!("OK");
            } else {
                eprintln!("Could not connect. Status code: {:?}", response.status());
            }
        }
        Err(err) => {
            eprintln!("Error: {:?}", err);
        }
    }

    Ok(())
}
