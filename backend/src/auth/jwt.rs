use axum::http::StatusCode;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, Header, EncodingKey};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Claims {
    exp: usize,
    user_id: i32,
}

pub fn generate_user_token(uid: i32) -> Result<String, StatusCode> {
    let mut time = Utc::now();
    let expires_in = Duration::minutes(1);
    time += expires_in;

    let claims = Claims {
        exp: time.timestamp() as usize,
        user_id: uid,
    };

    let key = EncodingKey::from_secret(std::env::var("JWT_SECRET").unwrap().as_ref());

    encode(&Header::default(), &claims, &key).map_err(|err| StatusCode::INTERNAL_SERVER_ERROR)
}