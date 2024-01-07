use std::collections::HashMap;

use chrono::NaiveDate;
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
pub struct ChartResponse<T, U> {
    pub x: Vec<T>,
    pub y: Vec<U>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Vec<String>>,
}

pub fn convert_to_standings_responses(data: MRData<StandingTable>) -> ChartResponse<String, i32> {
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
    ChartResponse {
        x,
        y,
        color: Some(color),
    }
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

pub fn convert_to_lap_chart_responses(data: MRData<RaceTable>) -> Vec<LapLineChartData> {
    let laps = data.table.races.get(0).unwrap().laps.as_ref().unwrap();
    let mut map = HashMap::new();
    for lap in laps {
        for timing in &lap.timings {
            let driver_id = timing.driver_id.clone();
            let time = timing.time.clone();
            let lap = lap.number;
            let entry = map
                .entry(driver_id.clone())
                .or_insert(LapLineChartData::new(driver_id));
            entry.laps.push(lap);
            entry
                .laptime
                .push(convert_lap_time_text_to_f64(&time).unwrap());
        }
    }
    let mut vec = map
        .into_iter()
        .map(|(_, v)| v)
        .collect::<Vec<LapLineChartData>>();
    vec.sort_by(|a, b| a.driver_id.partial_cmp(&b.driver_id).unwrap());
    vec
}

// convert text formatted like "m:ss.SSS" to f64
pub fn convert_lap_time_text_to_f64(lap_time: &str) -> Result<f64, &str> {
    let parts: Vec<&str> = lap_time.split(':').collect();
    if parts.len() != 2 {
        return Err("Invalid time format");
    }

    let min_sec: Vec<&str> = parts[1].split('.').collect();
    if min_sec.len() != 2 {
        return Err("Invalid time format");
    }

    let minutes = match parts[0].parse::<f64>() {
        Ok(m) => m,
        Err(_) => return Err("Invalid minutes"),
    };

    let seconds = match min_sec[0].parse::<f64>() {
        Ok(s) => s,
        Err(_) => return Err("Invalid seconds"),
    };

    let milliseconds = match min_sec[1].parse::<f64>() {
        Ok(ms) => ms / 1000.0, // convert milliseconds to seconds
        Err(_) => return Err("Invalid milliseconds"),
    };

    Ok(minutes * 60.0 + seconds + milliseconds)
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PitstopResponse {
    pub driver_id: String,
    pub lap: i32,
    pub duration: f64,
    pub stop: i32,
}

pub fn convert_to_pit_stop_responses(data: MRData<RaceTable>) -> Vec<PitstopResponse> {
    let pitstops = data.table.races.get(0).unwrap().pitstops.as_ref().unwrap();
    let vec = pitstops
        .iter()
        .map(|pitstop| PitstopResponse {
            driver_id: pitstop.driver_id.clone(),
            lap: pitstop.lap,
            duration: pitstop.duration.parse().unwrap_or_default(),
            stop: pitstop.stop,
        })
        .collect::<Vec<PitstopResponse>>();
    vec
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
