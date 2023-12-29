use crate::models::RaceResponse;
use crate::queries::{RoundQuery, YearQuery};
use axum::extract::Query;
use axum::{http::StatusCode, Json};
use ergast_rust::api::{Path, URLParams};
use ergast_rust::ergast::Ergast;
use ergast_rust::models::{MRData, SeasonTable, StandingTable};
use serde_json::Value;

// basic handler that responds with a static string
pub async fn root() -> &'static str {
    "Hello, World"
}

// basic handler returns a JSON object from Ergast::seasons
pub async fn seasons_handler(
) -> Result<(StatusCode, Json<MRData<SeasonTable>>), (StatusCode, Json<&'static str>)> {
    let params = URLParams {
        limit: 100,
        offset: 0,
    };
    let result = Ergast::seasons(params).await;

    match result {
        Ok(seasons) => Ok((StatusCode::OK, Json(seasons))),
        Err(_) => Err((StatusCode::BAD_REQUEST, Json("error"))),
    }
}

// handler returns a JSON object from Ergast::race
pub async fn races_handler(
    year: Query<YearQuery>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<&'static str>)> {
    let params = URLParams {
        limit: 100,
        offset: 0,
    };
    let result = Ergast::race(year.year, params).await; // Modify this line to pass the required 'year' argument

    match result {
        Ok(races) => {
            let response: Vec<RaceResponse> = races
                .table
                .races
                .iter()
                .map(|entity| RaceResponse {
                    season: year.year,
                    round: entity.round,
                    race_name: entity.race_name.clone(),
                    circuit_name: entity.circuit.circuit_name.clone(),
                    date: entity.date.clone(),
                })
                .collect();
            let value = serde_json::to_value(response).unwrap();
            Ok((StatusCode::OK, Json(value)))
        }
        Err(_) => Err((StatusCode::BAD_REQUEST, Json("error"))),
    }
}

// basic handler that responds with a static string
pub async fn standings_handler(
    round: Query<RoundQuery>,
) -> Result<(StatusCode, Json<MRData<StandingTable>>), (StatusCode, Json<&'static str>)> {
    let path = Path {
        year: round.year,
        round: Some(round.round),
    };
    let params = URLParams {
        limit: 100,
        offset: 0,
    };
    let result = Ergast::standings(path, params).await;

    match result {
        Ok(standings) => Ok((StatusCode::OK, Json(standings))),
        Err(_) => Err((StatusCode::BAD_REQUEST, Json("error"))),
    }
}
