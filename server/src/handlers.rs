use crate::db::connection::Pool;
use crate::db::db_models::{self, Season};
use crate::queries::{RoundQuery, YearQuery, LapChartQuery};
use axum::extract::Query;
use axum::response::IntoResponse;
use axum::Extension;
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
    Extension(pool): Extension<Pool>,
    year: Query<YearQuery>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<&'static str>)> {
    let mut conn = pool.get().expect("Failed to get DB connection from pool");
    let season = db_models::Season::get(year.year, &mut conn);

    let result = db_models::Race::generate_response(&season, &mut conn).await;
    let value = serde_json::to_value(result).unwrap();
    Ok((StatusCode::OK, Json(value)))
}

// basic handler that responds with a static string
pub async fn standings_handler(
    Extension(pool): Extension<Pool>,
    round: Query<RoundQuery>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<&'static str>)> {
    let mut conn = pool.get().expect("Failed to get DB connection from pool");
    let season = Season::get(round.year, &mut conn);
    let race = db_models::Race::get(&season, round.round, &mut conn);

    let race = match race {
        Some(r) => r,
        None => {
            return Err((StatusCode::BAD_REQUEST, Json("error")));
        }
    };

    let result = db_models::Standing::generate_response(&race, &mut conn).await;
    let value = serde_json::to_value(result).unwrap();
    Ok((StatusCode::OK, Json(value)))
}

pub async fn results_handler(
    Extension(pool): Extension<Pool>,
    round: Query<RoundQuery>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<&'static str>)> {
    let mut conn = pool.get().expect("Failed to get DB connection from pool");
    let season = Season::get(round.year, &mut conn);
    let race = db_models::Race::get(&season, round.round, &mut conn);
    let race = match race {
        Some(r) => r,
        None => {
            return Err((StatusCode::BAD_REQUEST, Json("error")));
        }
    };
    let result = db_models::RaceResult::generate_response(&race, &mut conn).await;
    let value = serde_json::to_value(result).unwrap();
    Ok((StatusCode::OK, Json(value)))
}

pub async fn seasons_handler(
    Extension(pool): Extension<Pool>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<&'static str>)> {
    let mut conn = pool.get().expect("Failed to get DB connection from pool");
    let result = db_models::Season::generate_response(&mut conn).await;

    match result {
        Ok(seasons) => {
            let value = serde_json::to_value(seasons).unwrap();
            Ok((StatusCode::OK, Json(value)))
        }
        Err(_) => Err((StatusCode::BAD_REQUEST, Json("error"))),
    }
}

pub async fn seasons_post(Extension(pool): Extension<Pool>) -> impl IntoResponse {
    let mut conn = pool.get().expect("Failed to get DB connection from pool");
    let _ = crate::db::db_models::Season::post(&mut conn).await;
    (StatusCode::OK, Json("ok"))
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

pub async fn laps_chart_handler(
    Extension(pool): Extension<Pool>,
    query: Query<LapChartQuery>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<&'static str>)> {
    let mut conn = pool.get().expect("Failed to get DB connection from pool");
    let season = Season::get(query.year, &mut conn);
    let race = db_models::Race::get(&season, query.round, &mut conn);

    let race = match race {
        Some(r) => r,
        None => {
            return Err((StatusCode::BAD_REQUEST, Json("error")));
        }
    };
    let result = db_models::Laptime::generate_response(&race, query.exclude_pitstop, &mut conn).await;
    let value = serde_json::to_value(result).unwrap();
    Ok((StatusCode::OK, Json(value)))
}

pub async fn pitstops_handler(
    Extension(pool): Extension<Pool>,
    round: Query<RoundQuery>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<&'static str>)> {
    let mut conn = pool.get().expect("Failed to get DB connection from pool");
    let season = Season::get(round.year, &mut conn);
    let race = db_models::Race::get(&season, round.round, &mut conn).unwrap();
    let result = db_models::Pitstop::generate_response(&race, &mut conn).await;
    let value = serde_json::to_value(result).unwrap();
    Ok((StatusCode::OK, Json(value)))
}

pub async fn drivers_get(
    Extension(pool): Extension<Pool>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<&'static str>)> {
    let mut conn = pool.get().expect("Failed to get DB connection from pool");
    let result = db_models::Driver::generate_response(&mut conn);
    let value = serde_json::to_value(result).unwrap();
    Ok((StatusCode::OK, Json(value)))
}

pub async fn drivers_post(Extension(pool): Extension<Pool>) -> impl IntoResponse {
    let mut conn = pool.get().expect("Failed to get DB connection from pool");
    let _ = crate::db::db_models::Driver::post(&mut conn).await;
    (StatusCode::OK, Json("ok"))
}

pub async fn constructors_get(
    Extension(pool): Extension<Pool>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<&'static str>)> {
    let mut conn = pool.get().expect("Failed to get DB connection from pool");
    let result = db_models::Constructor::generate_response(&mut conn);
    let value = serde_json::to_value(result).unwrap();
    Ok((StatusCode::OK, Json(value)))
}

pub async fn constructors_post(Extension(pool): Extension<Pool>) -> impl IntoResponse {
    let mut conn = pool.get().expect("Failed to get DB connection from pool");
    let _ = crate::db::db_models::Constructor::post(&mut conn).await;
    (StatusCode::OK, Json("ok"))
}
