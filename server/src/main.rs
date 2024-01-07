use axum::{routing::get, Router};
use http::Method;
use tower_http::cors::{Any, CorsLayer};

mod handlers;
use handlers::{
    laps_chart_handler, laps_handler, races_handler, results_handler, root, seasons_handler,
    standings_handler,
};

use crate::handlers::{pitstops_handler, seasons_post};
mod color_pallet;
mod db;
mod models;
mod queries;

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
        .route("/standings", get(standings_handler))
        .route("/seasons", get(seasons_handler).post(seasons_post))
        .route("/races", get(races_handler))
        .route("/results", get(results_handler))
        .route("/laps", get(laps_handler))
        .route("/laps-chart", get(laps_chart_handler))
        .route("/pitstops", get(pitstops_handler))
        .layer(cors);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(port).await.unwrap();
    println!("server is listening port http://{}", port);
    axum::serve(listener, app).await.unwrap();
}
