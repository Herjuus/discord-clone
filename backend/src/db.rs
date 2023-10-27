use lazy_static::lazy_static;
use libsql_client::{Client, Config};

lazy_static! {
    pub static ref CLIENT: Client = {
        let config = Config {
            url: url::Url::parse(std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.")).unwrap(),
            auth_token: None,
        };
        Client::from_config(config).await.unwrap()
    };
}