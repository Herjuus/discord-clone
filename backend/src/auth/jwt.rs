use axum::http::StatusCode;
use axum::Json;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, Header, EncodingKey, decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use crate::auth::User;
use crate::db::db_pool;

#[derive(Serialize, Deserialize)]
struct Claims {
    exp: usize,
    user_id: i32,
}

pub fn generate_user_token(uid: i32) -> Result<String, StatusCode> {
    let mut time = Utc::now();
    let expires_in = Duration::minutes(30);
    time += expires_in;

    let claims = Claims {
        exp: time.timestamp() as usize,
        user_id: uid,
    };

    let key = EncodingKey::from_secret(std::env::var("JWT_SECRET").unwrap().as_ref());

    encode(&Header::default(), &claims, &key).map_err(|e| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn validate_user_token(token: &str) -> Result<bool, (StatusCode, String)> {
    let pool = db_pool().await;

    let decoded_token = decode::<Claims>(&token, &DecodingKey::from_secret(std::env::var("JWT_SECRET").unwrap().as_ref()), &Validation::new(Algorithm::HS256))
        .map_err(|e| (StatusCode::UNAUTHORIZED, match e.to_string().as_str() {
            "ExpiredSignature" => {"Token expired.".to_string()},
            _ => {"Invalid token.".to_string()}
        }))?;

    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = ?", decoded_token.claims.user_id)
        .fetch_one(&pool)
        .await
        .map_err((StatusCode::UNAUTHORIZED, "Invalid token.".to_string()))?;

    Ok(true)
}