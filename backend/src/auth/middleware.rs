use axum::body::HttpBody;
use axum::http::{Request, StatusCode};
use axum::http::header::AUTHORIZATION;
use axum::middleware::Next;
use axum::response::Response;
use sqlx::{MySql, MySqlPool, Pool};
use crate::auth::jwt::validate_user_token;
use crate::error;
use crate::error::ApiError;
use crate::Tx;

pub async fn jwt_middleware<T>(mut req: Request<T>, next: Next<T>, mut tx: Tx) -> Result<Response, ApiError> {
    let db_pool = req.extensions().get::<MySqlPool>()
        .ok_or(ApiError { status_code: StatusCode::INTERNAL_SERVER_ERROR, message: "Could not connect to the database.".to_string() })?;

    let token = req.headers()
        .get(AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok() )
        .and_then(|auth_value| {
            if auth_value.starts_with("Bearer ") {
                Some(auth_value[7..].to_owned())
            } else {
                None
            }
        });

    let token = token.ok_or_else(|| {
        return Err(ApiError { status_code: StatusCode::UNAUTHORIZED, message: "No token provided.".to_string() })
    }).map_err(|e: Result<T, ApiError>| ApiError { status_code: StatusCode::UNAUTHORIZED, message: "No token provided.".to_string() })?;

    let user = validate_user_token(token.as_str(), db_pool.clone()).await?;

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}