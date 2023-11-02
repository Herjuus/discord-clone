use axum::http::StatusCode;

pub fn internal_error<E>(err: E) -> (StatusCode, String)
    where
        E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

pub fn login_error<E>(err: E) -> (StatusCode, String)
    where
        E: std::error::Error,
{
    (StatusCode::NOT_FOUND, "User with specified mail doesnt exist.".to_string())
}

pub fn register_error<E>(err: E) -> (StatusCode, String)
    where
        E: std::error::Error,
{
    (StatusCode::NOT_ACCEPTABLE, "User with these credentials already exists.".to_string())
}