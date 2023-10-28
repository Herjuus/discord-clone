mod auth;
mod lib;
use std::string::ToString;
use std::sync::Mutex;
use axum::{
    routing::get, routing::post,
    Router,
};
use dotenv::dotenv;
use discord_backend::run_database;

#[tokio::main]
async fn main(){
    dotenv().ok();
    run_database().await;

    let app = Router::new()
        .route("/register", post(auth::register_user))
        .route("/validate", post(auth::validate));

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}