use std::sync::{Arc, Mutex};

pub type DBClient = Arc<Mutex<libsql_client::Client>>;

pub async fn create_client(url: String, token: Option<String>) -> libsql_client::Client {
    libsql_client::Client::from_config(libsql_client::Config {
        url: url::Url::parse(&url).unwrap(),
        auth_token: token,
    })
    .await
    .unwrap()
}

fn escape_sql_string(s: &str) -> String {
    s.replace("'", "''") // Escape single quotes by doubling them
}

pub mod migrations;

pub mod messages;

pub mod recipes;

pub mod items;
