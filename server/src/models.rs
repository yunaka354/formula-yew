use chrono::NaiveDate;
use ergast_rust::models::{MRData, RaceTable};
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
pub struct ChartResponse<T, U> {
    pub x: Vec<T>,
    pub y: Vec<U>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LapLineChartData {
    pub driver_id: String,
    pub laps: Vec<i32>,
    pub laptime: Vec<f64>,
    pub color: String,
}

impl LapLineChartData {
    pub fn new(driver_id: String) -> Self {
        Self {
            driver_id: driver_id.clone(),
            laps: Vec::new(),
            laptime: Vec::new(),
            color: ColorPallet::get_color(&driver_id).to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PitstopResponse {
    pub driver_id: String,
    pub lap: i32,
    pub duration: f64,
    pub stop: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DriverResponse {
    pub id: String,
    pub permanent_number: Option<i32>,
    pub code: Option<String>,
    pub given_name: String,
    pub family_name: String,
    pub date_of_birth: NaiveDate,
    pub nationality: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ConstructorResponse {
    pub id: String,
    pub url: String,
    pub name: String,
    pub nationality: String,
}
