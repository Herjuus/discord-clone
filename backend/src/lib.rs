use sea_orm::{Database, DatabaseConnection};

pub async fn run_database() {
    let database = Database::connect(std::env::var("DATABASE_URL")).await;
}