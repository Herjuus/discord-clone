use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use crate::Tx;
use pwhash::bcrypt;

pub async fn register_user(mut tx: Tx, Json(payload): Json<Request>) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1234,
        username: payload.username,
        email: payload.email,
        hashed_password: bcrypt::hash(payload.password).unwrap(),
    };

    

    (StatusCode::CREATED, Json(user))
}

#[derive(Deserialize)]
pub struct Request {
    username: String,
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct User {
    id: i32,
    username: String,
    email: String,
    hashed_password: String,
}