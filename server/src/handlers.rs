use axum::{http::StatusCode, Json};
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

pub async fn standings_handler() -> (StatusCode, Json<User>) {
    let user = User {
        id: 1,
        username: "test".to_string(),
    };
    (StatusCode::OK, Json(user))
}
