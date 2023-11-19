mod auth;
mod lib;
pub mod error;

use std::string::ToString;
use axum::{Router, routing::get, routing::post};
use dotenv::dotenv;
use std::error::Error;
use axum::body::HttpBody;
use axum::middleware::from_fn;
use sqlx::MySqlPool;
use axum_analytics::Analytics;
use crate::auth::middleware::jwt_middleware;
use crate::auth::routes::auth_routes;

pub type Tx = axum_sqlx_tx::Tx<sqlx::MySql>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let pool = MySqlPool::connect(std::env::var("DATABASE_URL").unwrap().as_str()).await?;

    let app = Router::new()
        .nest("/auth", auth_routes())
        .layer(axum_sqlx_tx::Layer::new(pool.clone()));

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
