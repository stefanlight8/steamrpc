use std::env;
use std::error::Error;

use tokio_stream::StreamExt;
use tokio_stream::wrappers::IntervalStream;
use tokio::time::{interval,Duration};
use dotenv::dotenv;
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct PlayerSummaries {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting");
    dotenv().ok();

    let api_key: String = env::var("STEAM_API_KEY").expect("Steam API key is not defined");
    let profile_id: String = env::var("STEAM_PROFILE_ID").expect("Steam API key is not defined");
    
    let client = Client::new();
    let mut stream = IntervalStream::new(interval(Duration::from_secs(5)));

    while let Some(_) = stream.next().await {
        let data = match client.get(
            format!(
                "https://api.steampowered.com/ISteamUser/GetPlayerSummaries/v0002/?key={}&steamids={}",
                api_key, profile_id
            )
        ).send().await {
            Ok(response) => {
                let result = response.json::<PlayerSummaries>()
                    .await
                    .expect("Failed to deserialize response");
                // TODO: continue
            }
            Err(e) => println!("Failed to send request: {}", e)
        };


    };

    println!("Bye, bye!");
    Ok(())
}
