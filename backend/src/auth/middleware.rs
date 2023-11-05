use axum::http::{Request, StatusCode};
use axum::http::header::AUTHORIZATION;
use axum::middleware::Next;
use axum::response::Response;
use sqlx::MySqlPool;
use crate::auth::jwt::validate_user_token;
use crate::error::db_error;

pub async fn jwt_middleware<T>(mut req: Request<T>, next: Next<T>) -> Result<Response, (StatusCode, String)> {
    let db_pool = req.extensions().get::<MySqlPool>()
        .ok_or(db_error)?.to_owned();

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
        return Err((StatusCode::UNAUTHORIZED, "No token provided".to_string()))
    })?;

    let token_validated = validate_user_token(token.as_str(), db_pool).await?;

    if !token_validated {
        return Err((StatusCode::UNAUTHORIZED, "Invalid token.".to_string()))
    }

    Ok(next.run(req).await)
}