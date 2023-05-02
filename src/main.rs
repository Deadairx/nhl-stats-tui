use reqwest::Error;
use serde::Deserialize;
use tokio;

const SPORTS_DATA_IO_NHL_API_KEY: &str = "SPORTS_DATA_IO_NHL_KEY";

#[tokio::main]
async fn main() {
    let dallas_players = get_players_by_team("DAL".to_string()).await.unwrap();

    println!("{:?}", dallas_players);
}

async fn get_players_by_team(team_key: String) -> Result<Vec<Player>, Error> {
    let api_endpoint = format!("https://api.sportsdata.io/v3/nhl/scores/json/Players/{}?key={}", team_key, SPORTS_DATA_IO_NHL_API_KEY);

    let response: Vec<Player> = reqwest::get(api_endpoint).await?.json().await?;

    Ok(response)
}

#[derive(Debug, Deserialize)]
struct Player {
    player_id: String,
    first_name: String,
    last_name: String,
    status: PlayerStatus,
    team: Team,
    position: PlayerPosition,
    catches: Option<Hand>,
    shoots: Option<Hand>,
    /// in inches
    height: usize,
    /// in pounds
    weight: usize,
    photo_url: String,
}

#[derive(Debug, Deserialize)]
enum PlayerPosition {
    Goalie,
    Center,
    RightWing,
    LeftWing,
    Defense,
}

#[derive(Debug, Deserialize)]
enum Hand {
    Left,
    Right,
}

#[derive(Debug, Deserialize)]
enum PlayerStatus {
    /// Is playing for the team
    Active,
    /// Player is on the associated Minor league team
    Minor,
    Inactive
}

#[derive(Debug, Deserialize)]
enum Conference {
    Western,
    Eastern,
}

#[derive(Debug, Deserialize)]
enum Division {
    Atlantic,
    Metropolitan,
    Central,
    Pacific,
}

#[derive(Debug, Deserialize)]
struct TeamColors {
    primary: String,
    secondary: String,
    tertiary: Option<String>,
    quaternary: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Team {
    team_id: usize,
    team_key: String,
    active: bool,
    city: String,
    name: String,
    stadium_id: usize,
    conference: Conference,
    division: Division,
    colors: TeamColors,
    wiki_logo_url: String,
    global_team_id: usize
}
