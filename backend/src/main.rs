mod auth;

use std::string::ToString;
use std::sync::Mutex;
use axum::{
    routing::get, routing::post,
    Router,
};
use dotenv::dotenv;
use once_cell::sync::Lazy;

static CLIENT: Lazy<Mutex<libsql_client>> = Lazy::new(|| {
    let db = libsql_client::Client::from_config(libsql_client::Config {
        url: url::Url::parse(std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.").as_ref()).unwrap(),
        auth_token: Option::from(std::env::var("DATABASE_TOKEN").expect("DATABASE_URL must be set.")),
    });

    Mutex::new(db);
})



#[tokio::main]
async fn main(){
    dotenv().ok();

    CLIENT.lock().unwrap();

    let app = Router::new()
        .route("/register", post(auth::register_user))
        .route("/validate", post(auth::validate));

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}