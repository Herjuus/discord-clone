mod auth;
mod lib;
pub mod error;
mod db;

use std::string::ToString;
use axum::{Router, routing::get, routing::post};
use dotenv::dotenv;
use std::error::Error;
use crate::db::db_pool;

pub type Tx = axum_sqlx_tx::Tx<sqlx::MySql>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let pool = db_pool().await;

    let auth_routes = Router::new()
        .route("/register", post(auth::routes::register::register_user))
        .route("/login", post(auth::routes::login::login_user))
        .route("/validate", post(auth::validate))
        .route("/get_users", get(auth::get_users))
        .route("/validate-token", post(auth::validate_token));

    let app = Router::new()
        .nest("/auth", auth_routes)
        .layer(axum_sqlx_tx::Layer::new(pool.clone()));

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
