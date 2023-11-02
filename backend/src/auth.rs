mod jwt;
pub mod routes;

use std::error::Error;
use axum::{http::StatusCode, Json};
use axum::body::HttpBody;
use axum::response::IntoResponse;
use pwhash::bcrypt;
use serde::{Deserialize, Serialize};
use crate::{error, Tx};
use crate::auth::jwt::validate_user_token;

pub async fn get_users(mut tx: Tx) -> Result<(StatusCode, Json<Vec<User>>), (StatusCode, String)> {
    let users = sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(&mut tx)
        // .await.map_err(error::internal_error)
        .await.map_err(error::internal_error)?;

    Ok((StatusCode::OK, Json(users)))
}

#[derive(Serialize)]
pub struct User {
    id: i32,
    username: String,
    email: String,
    hashed_password: String,
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


pub async fn validate_token(Json(payload): Json<ValidateToken>) -> Result<Json<bool>, (StatusCode, String)>{
    let validated = validate_user_token(payload.token.as_str())?;

    Ok(Json(validated))
}

#[derive(Deserialize)]
pub struct ValidateToken {
    token: String,
}
