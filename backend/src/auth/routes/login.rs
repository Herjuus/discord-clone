use axum::http::StatusCode;
use axum::{debug_handler, Json};
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use crate::error::DbError;
use crate::Tx;
use pwhash::bcrypt;
use serde::de::Unexpected::Option;
use sqlx::Error;

struct User {
    id: i32,
    username: String,
    email: String,
    hashed_password: String,
}

// #[debug_handler]
pub async fn login_user(mut tx: Tx, Json(payload): Json<Request>) -> Result<(StatusCode, Json<Return>), DbError> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE email = ?", payload.email)
        .fetch_one(&mut tx)
        .await?;

    let correct_password = bcrypt::verify(payload.password, user.hashed_password.as_str());

    if !correct_password {
        // return Ok((StatusCode::BAD_REQUEST))
        return Err()
    }

    let return_object = Return {
        token: "".to_string(),
        message: "Logged in.".to_string(),
    };

    Ok((StatusCode::OK, Json(return_object)))
}

#[derive(Deserialize)]
pub struct Request {
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct Return {
    token: String,
    message: String,
}
