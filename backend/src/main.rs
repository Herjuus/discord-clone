mod auth;
mod lib;
mod error;

use std::string::ToString;
use axum::{Router, routing::get, routing::post};
use dotenv::dotenv;
use std::error::Error;
use sqlx::MySqlPool;

pub type Tx = axum_sqlx_tx::Tx<sqlx::MySql>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let pool = MySqlPool::connect(std::env::var("DATABASE_URL").unwrap().as_str()).await?;

    let app = Router::new()
        .route("/register", post(auth::routes::register::register_user))
        .route("/validate", post(auth::validate))
        .route("/getusers", get(auth::get_users))
        .route("/login", post(auth::routes::login::login_user))
        .layer(axum_sqlx_tx::Layer::new(pool.clone()));

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
