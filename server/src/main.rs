use axum::{http::StatusCode, routing::get, Json, Router};
use http::Method;
use serde::Serialize;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();
    // CORS setting
    // CAUTION: change this setting for production.
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(Any)
        .allow_origin(Any);

    let port = "0.0.0.0:3000";

    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .route("/user", get(user_handler))
        .layer(cors);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(port).await.unwrap();
    println!("server is listening port {:?}", port);
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World"
}

async fn user_handler() -> (StatusCode, Json<User>) {
    let user = User {
        id: 1,
        username: "test".to_string(),
    };
    (StatusCode::OK, Json(user))
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
