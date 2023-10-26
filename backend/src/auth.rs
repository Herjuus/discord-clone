use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use pwhash::bcrypt;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

pub async fn register_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    // let hash = bcrypt

    let user = User {
        id: 1234,
        username: payload.username,
        email: payload.email,
        hashed_password: bcrypt::hash(payload.password).unwrap(),
    };

    (StatusCode::CREATED, Json(user))
}

#[derive(Deserialize)]
pub struct CreateUser {
    username: String,
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct User {
    id: u64,
    username: String,
    email: String,
    hashed_password: String
}