use serde::{Deserialize, Deserializer};

fn deserialize_non_empty_players<'de, D>(deserializer: D) -> Result<Vec<Player>, D::Error>
where
    D: Deserializer<'de>,
{
    let players: Vec<Player> = Deserialize::deserialize(deserializer)?;
    Ok(players.into_iter().filter(|p| p.gameextrainfo.is_some() || p.gameid.is_some()).collect())
}


#[derive(Deserialize)]
pub struct Player {
    pub gameextrainfo: Option<String>,
    pub gameid: Option<String>,
}

#[derive(Deserialize)]
pub struct PlayerSummariesResponse {
    #[serde(deserialize_with = "deserialize_non_empty_players")]
    pub players: Vec<Player>,
}

#[derive(Deserialize)]
pub struct PlayerSummaries {
    pub response: PlayerSummariesResponse,
}