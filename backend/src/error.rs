use axum::http::StatusCode;
use axum::response::IntoResponse;

pub struct DbError(sqlx::Error);

impl From<sqlx::Error> for DbError {
    fn from(error: sqlx::Error) -> Self {
        Self(error)
    }
}

// impl IntoResponse for DbError {
//     fn into_response(self) -> axum::response::Response {
//         println!("ERROR: {}", self.0);
//         (StatusCode::INTERNAL_SERVER_ERROR, "internal server error").into_response()
//     }
// }