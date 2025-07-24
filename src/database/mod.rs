use libsql_orm::Database;
use std::sync::{Arc, Mutex};

pub type DBClient2 = Arc<Mutex<libsql_orm::Database>>;

pub async fn create_orm_client(url: String, token: Option<String>) -> libsql_orm::Database {
    let token = token.unwrap_or_default();
    Database::new_connect(&url, &token).await.unwrap()
}

pub mod migrations;

pub mod messages;

pub mod recipes;

pub mod items;
