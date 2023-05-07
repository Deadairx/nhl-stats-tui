// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::Players;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: Players = serde_json::from_str(&json).unwrap();
// }

use core::fmt;

use serde::{Serialize, Deserialize};

pub type Players = Vec<Player>;

#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    #[serde(rename = "PlayerID")]
    pub player_id: i64,

    #[serde(rename = "FirstName")]
    pub first_name: String,

    #[serde(rename = "LastName")]
    pub last_name: String,

    #[serde(rename = "Status")]
    pub status: Status,

    #[serde(rename = "TeamID")]
    team_id: i64,

    #[serde(rename = "Team")]
    team: Team,

    #[serde(rename = "Position")]
    pub position: Position,

    #[serde(rename = "Jersey")]
    pub jersey: Option<i64>,

    #[serde(rename = "Catches")]
    catches: Option<Catches>,

    #[serde(rename = "Shoots")]
    pub shoots: Option<Catches>,

    #[serde(rename = "Height")]
    pub height: i64,

    #[serde(rename = "Weight")]
    pub weight: i64,

    #[serde(rename = "BirthDate")]
    pub birth_date: String,

    #[serde(rename = "BirthCity")]
    pub birth_city: String,

    #[serde(rename = "BirthState")]
    pub birth_state: Option<String>,

    #[serde(rename = "PhotoUrl")]
    pub photo_url: String,

    #[serde(rename = "SportRadarPlayerID")]
    sport_radar_player_id: String,

    #[serde(rename = "RotoworldPlayerID")]
    rotoworld_player_id: Option<i64>,

    #[serde(rename = "RotoWirePlayerID")]
    roto_wire_player_id: Option<i64>,

    #[serde(rename = "FantasyAlarmPlayerID")]
    fantasy_alarm_player_id: Option<i64>,

    #[serde(rename = "StatsPlayerID")]
    stats_player_id: Option<i64>,

    #[serde(rename = "SportsDirectPlayerID")]
    sports_direct_player_id: Option<i64>,

    #[serde(rename = "XmlTeamPlayerID")]
    xml_team_player_id: Option<i64>,

    #[serde(rename = "InjuryStatus")]
    injury_status: Injury,

    #[serde(rename = "InjuryBodyPart")]
    injury_body_part: Injury,

    #[serde(rename = "InjuryStartDate")]
    injury_start_date: Option<serde_json::Value>,

    #[serde(rename = "InjuryNotes")]
    injury_notes: Injury,

    #[serde(rename = "FanDuelPlayerID")]
    fan_duel_player_id: Option<i64>,

    #[serde(rename = "DraftKingsPlayerID")]
    draft_kings_player_id: Option<i64>,

    #[serde(rename = "YahooPlayerID")]
    yahoo_player_id: Option<i64>,

    #[serde(rename = "FanDuelName")]
    fan_duel_name: Option<String>,

    #[serde(rename = "DraftKingsName")]
    draft_kings_name: Option<String>,

    #[serde(rename = "YahooName")]
    yahoo_name: Option<String>,

    #[serde(rename = "DepthChartPosition")]
    depth_chart_position: Option<Position>,

    #[serde(rename = "DepthChartOrder")]
    depth_chart_order: Option<i64>,

    #[serde(rename = "GlobalTeamID")]
    global_team_id: i64,

    #[serde(rename = "FantasyDraftName")]
    fantasy_draft_name: Option<String>,

    #[serde(rename = "FantasyDraftPlayerID")]
    fantasy_draft_player_id: Option<i64>,

    #[serde(rename = "UsaTodayPlayerID")]
    usa_today_player_id: Option<i64>,

    #[serde(rename = "UsaTodayHeadshotUrl")]
    usa_today_headshot_url: Option<String>,

    #[serde(rename = "UsaTodayHeadshotNoBackgroundUrl")]
    usa_today_headshot_no_background_url: Option<String>,

    #[serde(rename = "UsaTodayHeadshotUpdated")]
    usa_today_headshot_updated: Option<String>,

    #[serde(rename = "UsaTodayHeadshotNoBackgroundUpdated")]
    usa_today_headshot_no_background_updated: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Catches {
    #[serde(rename = "-")]
    Empty,

    #[serde(rename = "L")]
    Left,

    #[serde(rename = "R")]
    Right,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Position {
    #[serde(rename = "C")]
    Center,

    #[serde(rename = "D")]
    Defense,

    #[serde(rename = "G")]
    Goalie,

    #[serde(rename = "LW")]
    LeftWing,

    #[serde(rename = "RW")]
    RightWing,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Position::Center => write!(f, "Center"),
            Position::Defense => write!(f, "Defense"),
            Position::Goalie => write!(f, "Goalie"),
            Position::LeftWing => write!(f, "LeftWing"),
            Position::RightWing => write!(f, "RightWing"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Injury {
    #[serde(rename = "Scrambled")]
    Scrambled,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Status {
    #[serde(rename = "Active")]
    Active,

    #[serde(rename = "Minors")]
    Minors,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Team {
    #[serde(rename = "DAL")]
    Dal,
}
