use axum::http::StatusCode;
use axum::{debug_handler, Json};
use axum::body::HttpBody;
use serde::{Deserialize, Serialize};
use crate::error;
use crate::Tx;
use pwhash::bcrypt;

pub async fn login_user(mut tx: Tx, Json(payload): Json<Request>) -> Result<(StatusCode, Json<Return>), (StatusCode, String)> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE email = ?", payload.email)
        .fetch_one(&mut tx)
        .await.map_err(error::login_error)?;

    let correct_password = bcrypt::verify(payload.password, user.hashed_password.as_str());

    if !correct_password {
        return Err((StatusCode::BAD_REQUEST, "Wrong password.".to_string()))
    }

    let return_object = Return {
        token: "".to_string(),
        message: "Logged in.".to_string(),
    };

    Ok((StatusCode::OK, Json(return_object)))
}

struct User {
    id: i32,
    username: String,
    email: String,
    hashed_password: String,
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