use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct RaceResponse {
    pub season: i32,
    pub round: i32,
    pub race_name: String,
    pub circuit_name: String,
    pub date: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ResultResponse {
    pub position: i32,
    pub position_text: String,
    pub code: String,
    pub given_name: String,
    pub family_name: String,
    pub points: f32,
    pub status: String,
    pub constructor: String,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct SeasonResponse {
    pub season: i32,
    pub url: String,
}
