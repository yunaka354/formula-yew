use crate::queries::Round;
use axum::extract::Query;
use axum::{http::StatusCode, Json};
use ergast_rust::api::{Path, URLParams};
use ergast_rust::ergast::Ergast;
use ergast_rust::models::{MRData, StandingTable};

// basic handler that responds with a static string
pub async fn root() -> &'static str {
    "Hello, World"
}

// basic handler that responds with a static string
pub async fn standings_handler(
    round: Query<Round>,
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
