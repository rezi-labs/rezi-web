use std::sync::{Arc, Mutex};

use libsql_orm::Database;

pub type DBClient2 = Arc<Mutex<libsql_orm::Database>>;

pub async fn create_orm_client(url: String, token: Option<String>) -> libsql_orm::Database {
    let token = token.unwrap_or_default();
    Database::new_connect(&url, &token).await.unwrap()
}

fn escape_sql_string(s: &str) -> String {
    s.replace("'", "''") // Escape single quotes by doubling them
}

pub mod migrations;

pub mod messages;

pub mod recipes;

pub mod items;
