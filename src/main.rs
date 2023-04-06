use serde::{Deserialize, Serialize};
use serde_json;
use csv;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MastersStats {
    data: Data,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Data {
    current_round: String,
    player: Vec<Player>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Player {
    first_name: String,
    last_name: String,
    thru: String,
    topar: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://www.masters.com/en_US/scores/feeds/2023/scores.json")
        .await?
        .json::<MastersStats>()
        .await?;

    std::fs::write("./player_data.json", serde_json::to_string(&resp)?)?;

    let mut wtr = csv::Writer::from_path("./player_data.csv")?;
    for player in resp.data.player {
        wtr.serialize(player)?;
    } 
    wtr.flush()?;

    Ok(())
}
