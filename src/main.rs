// Healh check monitoring
// @author Ipan Ardian

use reqwest::Client;
use std::time::Duration;
use tokio::time;
use toml::Value;

fn load_configuration() -> Result<Value, Box<dyn std::error::Error>> {
    let config_str = std::fs::read_to_string("config.toml")?;
    let config: Value = toml::from_str(&config_str)?;
    Ok(config)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the configuration
    let config = load_configuration()?;
    
    // Read the URL from the configuration file
    let url = match config["server"]["url"].as_str() {
        Some(u) => u,
        None => {
            eprintln!("Error: 'server.url' not found in the configuration");
            return Ok(());
        }
    };

    let interval = match config["healthcheck"]["interval"].as_integer() {
        Some(i) => i as u64,
        None => {
            eprintln!("Error: 'healthcheck.interval' not found in the configuration");
            return Ok(());
        }
    };

    let client = Client::new();

    loop {
        match client.get(url).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    // println!("OK");
                } else {
                    println!("Could not connect. Status code: {:?}", response.status());
                }
            }
            Err(err) => {
                println!("Error: {:?}", err);
            }
        }

        // Wait for some time before sending the next request
        time::sleep(Duration::from_secs(interval)).await;
    }
}
