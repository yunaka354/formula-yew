use axum::{routing::get, Router};
use db::db_models::{Season, Driver, Constructor};
use http::Method;
use tower_http::cors::{Any, CorsLayer};

mod handlers;
use handlers::{
    laps_chart_handler, laps_handler, races_handler, results_handler, root, seasons_handler,
    standings_handler,
};

use crate::handlers::{pitstops_handler, seasons_post, drivers_get, drivers_post, constructors_get, constructors_post};
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
        .route("/drivers", get(drivers_get).post(drivers_post))
        .route("/constructors", get(constructors_get).post(constructors_post))
        .layer(cors);

    // initial check function to ensure essential tables exist.
    check_and_create_tables().await;

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(port).await.unwrap();
    println!("server is listening port http://{}", port);
    axum::serve(listener, app).await.unwrap();
}

async fn check_and_create_tables() {
    if !Season::is_exist() {
        println!("Season data is not exist. Create season data.");
        Season::post().await;
    }
    
    if !Driver::is_exist() {
        println!("Driver data is not exist. Create driver data.");
        Driver::post().await;
    }

    if !Constructor::is_exist() {
        println!("Constructor data is not exist. Create constructor data.");
        Constructor::post().await;
    }
}