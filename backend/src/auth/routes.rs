pub mod register;
pub mod login;

use axum::Router;
use axum::middleware::from_fn;
use axum::routing::{get, post};
use crate::auth::get_users;
use crate::auth::middleware::jwt_middleware;

pub fn auth_routes() -> Router {
    let routes = Router::new()
        .route("/get_users", get(get_users))
        .route_layer(from_fn(jwt_middleware))
        .route("/register", post(register::register_user))
        .route("/login", post(login::login_user));
    (routes)
}