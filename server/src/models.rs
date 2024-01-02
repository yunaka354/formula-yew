use ergast_rust::models::{MRData, RaceTable, StandingTable};
use serde::{Deserialize, Serialize};

use crate::color_pallet::ColorPallet;

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

/// StandingsResponse provides a response for Plotly Bar Chart.
#[derive(Deserialize, Serialize, Debug)]
pub struct StandingsResponse {
    pub x: Vec<String>,
    pub y: Vec<i32>,
    pub color: Vec<String>,
}

pub fn convert_to_standings_responses(data: MRData<StandingTable>) -> StandingsResponse {
    let standings = data.table.standings_lists.get(0).unwrap();
    let mut x = Vec::new();
    let mut y = Vec::new();
    let mut color = Vec::new();
    for entity in standings.driver_standings.iter() {
        x.push(entity.driver.code.clone().unwrap());
        y.push(entity.points);
        color.push(ColorPallet::get_color(
            &entity.constructors.get(0).unwrap().name,
        ));
    }
    StandingsResponse { x, y, color }
}
