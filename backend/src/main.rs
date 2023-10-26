mod auth;

use axum::{
    routing::get, routing::post,
    Router,
};

#[tokio::main]
async fn main(){
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/register", post(auth::register_user));

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}