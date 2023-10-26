use axum::{
    http::StatusCode,
    Json,
};
use pwhash::bcrypt;
use serde::{Deserialize, Serialize};

pub async fn register_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    // let hash = bcrypt

    let pass = payload.password.clone();

    let user = User {
        id: 1234,
        username: payload.username,
        email: payload.email,
        hashed_password: bcrypt::hash(payload.password).unwrap(),
    };

    // assert_eq!(bcrypt::verify(pass, user.hashed_password.as_str()), true);

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
    hashed_password: String,
}

pub async fn validate(Json(payload): Json<ValidateUser>) -> (StatusCode, Json<ValidatedUser>) {

    let res = ValidatedUser {
        validated: bcrypt::verify(payload.password, payload.hashed_password.as_str())
    };

    (StatusCode::OK, Json(res))
}

#[derive(Deserialize)]
pub struct ValidateUser {
    password: String,
    hashed_password: String,
}

#[derive(Serialize)]
pub struct ValidatedUser {
    validated: bool,
}
