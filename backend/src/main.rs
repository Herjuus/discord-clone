mod auth;
mod db;
use std::string::ToString;
use std::sync::Mutex;
use axum::{
    routing::get, routing::post,
    Router,
};
use dotenv::dotenv;
use once_cell::sync::Lazy;

#[tokio::main]
async fn main(){
    dotenv().ok();

    let app = Router::new()
        .route("/register", post(auth::register_user))
        .route("/validate", post(auth::validate));

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}