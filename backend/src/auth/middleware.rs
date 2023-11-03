use std::any::Any;
use axum::http::{Request, StatusCode};
use axum::http::header::AUTHORIZATION;
use axum::middleware::Next;
use axum::response::Response;
use crate::auth::jwt::validate_user_token;

pub async fn jwt_middleware<T>(mut req: Request<T>, next: Next<T>) -> Result<Response, (StatusCode, String)> {
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
        (StatusCode::UNAUTHORIZED, "No token provided".to_string())
    })?;

    let token_validated = validate_user_token(token.as_str());

    Ok(next.run(req).await)
}