use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use crate::Tx;
use pwhash::bcrypt;
use crate::auth::error::DbError;

pub async fn register_user(mut tx: Tx, Json(payload): Json<Request>) -> Result<(StatusCode), (DbError)> {
    let user = User {
        username: payload.username,
        email: payload.email,
        hashed_password: bcrypt::hash(payload.password).unwrap(),
    };

    let result = sqlx::query(
        "INSERT INTO users (username, email, hashed_password) VALUES (?, ?, ?)")
        .bind(user.username)
        .bind(user.email)
        .bind(user.hashed_password)
        .execute(&mut tx).await?;

    Ok((StatusCode::OK))
}

#[derive(Deserialize)]
pub struct Request {
    username: String,
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct User {
    username: String,
    email: String,
    hashed_password: String,
}