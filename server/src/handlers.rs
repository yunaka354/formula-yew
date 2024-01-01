use crate::models::{RaceResponse, ResultResponse, SeasonResponse};
use crate::queries::{RoundQuery, YearQuery};
use axum::extract::Query;
use axum::{http::StatusCode, Json};
use ergast_rust::api::{Path, URLParams};
use ergast_rust::ergast::Ergast;
use serde_json::Value;

// basic handler that responds with a static string
pub async fn root() -> &'static str {
    "Hello, World"
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
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<&'static str>)> {
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
        Ok(standings) => {
            let response = crate::models::convert_to_standings_responses(standings);
            let value = serde_json::to_value(response).unwrap();
            Ok((StatusCode::OK, Json(value)))
        }
        Err(_) => Err((StatusCode::BAD_REQUEST, Json("error"))),
    }
}

pub async fn results_handler(
    round: Query<RoundQuery>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<&'static str>)> {
    let path = Path {
        year: round.year,
        round: Some(round.round),
    };
    let params = URLParams {
        limit: 100,
        offset: 0,
    };
    let result = Ergast::results(path, params).await;

    match result {
        Ok(results) => {
            let response: Vec<ResultResponse> = results
                .table
                .races
                .get(0)
                .map(|race| race.results.as_ref().unwrap())
                .unwrap()
                .iter()
                .map(|entity| ResultResponse {
                    position: entity.position,
                    position_text: entity.position_text.clone(),
                    code: entity.driver.code.clone().unwrap(),
                    given_name: entity.driver.given_name.clone(),
                    family_name: entity.driver.family_name.clone(),
                    points: entity.points,
                    status: entity.status.clone(),
                    constructor: entity.constructor.name.clone(),
                })
                .collect();
            let value = serde_json::to_value(response).unwrap();
            Ok((StatusCode::OK, Json(value)))
        }
        Err(_) => Err((StatusCode::BAD_REQUEST, Json("error"))),
    }
}

pub async fn seasons_handler() -> Result<(StatusCode, Json<Value>), (StatusCode, Json<&'static str>)>
{
    let params = URLParams {
        limit: 100,
        offset: 0,
    };
    let result = Ergast::seasons(params).await;

    match result {
        Ok(seasons) => {
            let mut response: Vec<SeasonResponse> = seasons
                .table
                .seasons
                .iter()
                .map(|entity| SeasonResponse {
                    season: entity.season.clone(),
                    url: entity.url.clone(),
                })
                .collect();
            response.reverse();
            let value = serde_json::to_value(response).unwrap();
            Ok((StatusCode::OK, Json(value)))
        }
        Err(_) => Err((StatusCode::BAD_REQUEST, Json("error"))),
    }
}

pub async fn laps_handler(
    round: Query<RoundQuery>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<&'static str>)> {
    let path = Path {
        year: round.year,
        round: Some(round.round),
    };
    let params = URLParams {
        limit: 2000,
        offset: 0,
    };
    let result = Ergast::laps(path, params).await;

    match result {
        Ok(laps) => {
            let response = crate::models::convert_to_lap_responses(laps);
            let value = serde_json::to_value(response).unwrap();
            Ok((StatusCode::OK, Json(value)))
        }
        Err(_) => Err((StatusCode::BAD_REQUEST, Json("error"))),
    }
}
