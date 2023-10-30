mod auth;
mod lib;
use std::string::ToString;
use std::sync::Mutex;
use axum::{routing::get, routing::post, Router, Extension};
use dotenv::dotenv;
use std::error::Error;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;

pub type Tx = axum_sqlx_tx::Tx<sqlx::MySql>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    // let pool = MySqlPoolOptions::new()
    //     .max_connections(5)
    //     .connect(std::env::var("DATABASE_URL").unwrap().as_str()).await?;

    let pool = MySqlPool::connect(std::env::var("DATABASE_URL").unwrap().as_str()).await?;

    let app = Router::new()
        .route("/register", post(auth::routes::register::register_user))
        .route("/validate", post(auth::validate))
        .route("/getusers", get(auth::get_users))
        .layer(axum_sqlx_tx::Layer::new(pool.clone()));

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}