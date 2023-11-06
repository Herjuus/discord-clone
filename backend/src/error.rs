use axum::http::{header, StatusCode};
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;

pub fn internal_error<E>(err: E) -> (StatusCode, String)
    where
        E: std::error::Error, { (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()) }

pub fn login_error<E>(err: E) -> (StatusCode, String)
    where
        E: std::error::Error,
{
    (StatusCode::NOT_FOUND, "User with specified email doesnt exist.".to_string())
}

pub fn register_error<E>(err: E) -> (StatusCode, String)
    where
        E: std::error::Error,
{
    (StatusCode::NOT_ACCEPTABLE, "User with these credentials already exists.".to_string())
}

pub fn db_error<E>(err: E) -> (StatusCode, String)
    where
        E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, "Could not connect to the database.".to_string())
}

#[derive(Debug)]
pub struct ApiError {
    pub status_code: StatusCode,
    pub message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let status_code = self.status_code;
        (status_code,[(header::CONTENT_TYPE,"application/json")],Json(json!({ "StatusCode": self.status_code.as_u16(), "Message": self.message }))).into_response()
    }
}