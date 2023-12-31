use ergast_rust::models::{MRData, RaceTable};
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

#[derive(Deserialize, Serialize, Debug)]
pub struct LapResponse {
    pub driver_id: String,
    pub position: i32,
    pub time: String,
    pub lap: i32,
}

pub fn convert_to_lap_responses(data: MRData<RaceTable>) -> Vec<LapResponse> {
    let laps = data.table.races.get(0).unwrap().laps.as_ref().unwrap();
    let mut vec = Vec::new();
    for lap in laps {
        for timing in &lap.timings {
            vec.push(LapResponse {
                driver_id: timing.driver_id.clone(),
                position: timing.position,
                time: timing.time.clone(),
                lap: lap.number,
            });
        }
    }
    vec
}
