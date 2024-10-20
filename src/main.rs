mod structs;
mod utils;

use std::{env, error::Error};

use chrono::Utc;
use discord_rich_presence::{
    activity::{Activity, Timestamps},
    DiscordIpc, DiscordIpcClient,
};
use dotenv::dotenv;
use reqwest::Client;
use tokio::time::{interval, Duration};
use tokio_stream::{wrappers::IntervalStream, StreamExt};

use structs::PlayerSummaries;
use utils::get_default_activity;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello");
    dotenv().ok();

    let http_client = Client::new();
    let mut rpc_client =
        DiscordIpcClient::new(&env::var("DISCORD_RPC_ID").expect("Discord RPC ID is not defined"))?;

    loop {
        match rpc_client.connect() {
            Ok(_) => {
                println!("Connected to Discord RPC");
                rpc_client.set_activity(get_default_activity())?;
                break;
            }
            Err(e) => eprintln!("Failed to connect to Discord RPC: {e}, retrying"),
        }
    }

    let mut stream = IntervalStream::new(interval(Duration::from_secs(5)));
    let mut timestamp: Option<i64> = None;

    while stream.next().await.is_some() {
        match http_client
            .get(format!(
            "https://api.steampowered.com/ISteamUser/GetPlayerSummaries/v0002/?key={}&steamids={}",
            env::var("STEAM_API_KEY").expect("Steam API key is not defined"), 
            env::var("STEAM_PROFILE_ID").expect("Steam Profile ID is not defined"),
        ))
            .send()
            .await
        {
            Ok(response) => {
                let data: PlayerSummaries = response.json().await?;
                if let Some(player) = data.response.players.first() {
                    if let Some(game_name) = player.gameextrainfo.as_deref() {
                        println!("Updated activity, now you playing {game_name}");
                        
                        if timestamp.is_none() {
                            timestamp = Some(Utc::now().timestamp())
                        }

                        rpc_client.set_activity(
                            Activity::new()
                                .timestamps(Timestamps::new().start(timestamp.unwrap()))  // unsafe
                                .state(game_name),
                        )?;
                    }
                } else {
                    rpc_client.set_activity(get_default_activity())?;
                    timestamp = None;
                }
            }
            Err(e) => eprintln!("Failed to send request: {e}"),
        }
    }
    Ok(())
}
