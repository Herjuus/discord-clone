mod jwt;

use std::error::Error;
use axum::{Extension, http::StatusCode, Json};
use pwhash::bcrypt;
use serde::{Deserialize, Serialize};
use crate::DB;

pub async fn get_users(mut db: DB) -> Result<(StatusCode, Json<Vec<User>>), DbError> {
    let users = sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(&mut db)
        .await?;

    Ok((StatusCode::OK, Json(users)))
}

struct DbError(sqlx::Error);

pub async fn register_user(mut db: DB, Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
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
    id: i32,
    username: String,
    email: String,
    hashed_password: String,
}

// pub async fn sign_in(Json(payload): Json<SignInUser>) -> Json<SignInReturn>{
//     let client = &*CLIENT;
//
//     let ret = SignInReturn {
//         token: "lol".to_string(),
//         message: "ok".to_string(),
//     };
// }

#[derive(Deserialize)]
pub struct SignInUser {
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct SignInReturn {
    token: String,
    message: String,
}

pub async fn validate(Json(payload): Json<ValidateUser>) -> (StatusCode, Json<ValidatedUser>) {

    let res = ValidatedUser {
        validated: bcrypt::verify(payload.password, payload.hashed_password.as_str())
    };

    if res.validated {
        (StatusCode::OK, Json(res))
    } else {
        (StatusCode::BAD_REQUEST, Json(res))
    }
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
