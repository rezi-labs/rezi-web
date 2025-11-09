use libsql_orm::Database;
use std::sync::{Arc, Mutex};

pub type DBClient = Arc<Mutex<DB>>;

pub struct DB {
    url: String,
    token: Option<String>,
}

impl DB {
    pub fn new(url: String, token: Option<String>) -> Self {
        DB { url, token }
    }

    pub async fn connect(&self) -> libsql_orm::Database {
        let token = self.token.clone().unwrap_or_default();
        Database::new_connect(&self.url, &token).await.unwrap()
    }
}

#[allow(clippy::await_holding_lock)]
pub async fn unlock_client(client: &DBClient) -> libsql_orm::Database {
    client.lock().unwrap().connect().await
}

pub async fn create_orm_client(url: String, token: Option<String>) -> DB {
    DB::new(url, token)
}

pub mod migrations;

pub mod recipes;

pub mod items;
