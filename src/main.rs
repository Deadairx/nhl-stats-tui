use std::env;

use reqwest::Error;
use tokio;
use crate::players::{Players, Position};

mod players;

const SPORTS_DATA_IO_NHL_API_KEY: &str = "SPORTS_DATA_IO_NHL_KEY";

#[tokio::main]
async fn main() {
    let dallas_players = get_players_by_team("DAL".to_string()).await.unwrap();

    println!("Forwards");
    println!("--------");
    dallas_players
        .iter()
        .filter(|p| matches!(p.position, Position::Center | Position::LeftWing | Position::RightWing))
        .for_each(|player| println!("{}, {} - {}", player.last_name, player.first_name, player.position));
    println!("");
    println!("Defense");
    println!("--------");
    dallas_players
        .iter()
        .filter(|p| matches!(p.position, Position::Defense))
        .for_each(|player| println!("{}, {} - {}", player.last_name, player.first_name, player.position));
    println!("");
    println!("Goalies");
    println!("--------");
    dallas_players
        .iter()
        .filter(|p| matches!(p.position, Position::Goalie))
        .for_each(|player| println!("{}, {} - {}", player.last_name, player.first_name, player.position));
}

async fn get_players_by_team(team_key: String) -> Result<Players, Error> {
    let api_endpoint = format!("https://api.sportsdata.io/v3/nhl/scores/json/Players/{}?key={}", team_key, env::var(SPORTS_DATA_IO_NHL_API_KEY).unwrap());

    let response = reqwest::get(api_endpoint).await?;
        
    let players: Players = response.json().await?;

    Ok(players)
}
