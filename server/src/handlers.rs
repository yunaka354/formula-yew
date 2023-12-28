use axum::{http::StatusCode, Json};
use ergast_rust::api::{Path, URLParams};
use ergast_rust::ergast::Ergast;
use ergast_rust::models::{MRData, StandingTable};
use serde::Serialize;

// the output to our `create_user` handler
#[derive(Serialize)]
pub struct User {
    id: u64,
    username: String,
}

// basic handler that responds with a static string
pub async fn root() -> &'static str {
    "Hello, World"
}

// basic handler that responds with a static string
pub async fn standings_handler(
) -> Result<(StatusCode, Json<MRData<StandingTable>>), (StatusCode, Json<&'static str>)> {
    let path = Path {
        year: 2023,
        round: Some(1),
    };
    let params = URLParams::default();
    let result = Ergast::standings(path, params).await;

    match result {
        Ok(standings) => Ok((StatusCode::OK, Json(standings))),
        Err(_) => Err((StatusCode::BAD_REQUEST, Json("error"))),
    }
}
