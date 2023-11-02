use axum::http::StatusCode;
use axum::Json;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, Header, EncodingKey, decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use crate::error;

#[derive(Serialize, Deserialize)]
struct Claims {
    exp: usize,
    user_id: i32,
}

pub fn generate_user_token(uid: i32) -> Result<String, StatusCode> {
    let mut time = Utc::now();
    let expires_in = Duration::seconds(10);
    time += expires_in;

    let claims = Claims {
        exp: time.timestamp() as usize,
        user_id: uid,
    };

    let key = EncodingKey::from_secret(std::env::var("JWT_SECRET").unwrap().as_ref());

    encode(&Header::default(), &claims, &key).map_err(|err| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn validate_user_token(token: &str) -> Result<bool, (StatusCode, String)> {
    let decoded_token = decode::<Claims>(&token, &DecodingKey::from_secret(std::env::var("JWT_SECRET").unwrap().as_ref()), &Validation::new(Algorithm::HS256))
        .map_err(error::internal_error)?;

    Ok(true)
}