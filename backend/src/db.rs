use libsql_client::Client;
use once_cell::sync::Lazy;

pub static CLIENT: Lazy<Client> = Lazy::new(|| {
    libsql_client::Client::from_config(libsql_client::Config {
        url: url::Url::parse(&*std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.")).unwrap(),
        auth_token: None,
    })
});
