use libsql_client::Statement;
use log::{error, info};

use actix_web::Result;
use chrono::{DateTime, Utc};
use libsql_client::Row;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::database::DBClient;

use crate::database::escape_sql_string;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: u32,
    pub content: String,
    pub ai_response: String,
    pub sender: String,
    pub timestamp: DateTime<Utc>,
    pub is_user: bool,
}

impl ChatMessage {
    pub fn ai_message(&self) -> Self {
        ChatMessage {
            id: self.id,
            content: self.ai_response.clone(),
            ai_response: self.ai_response.clone(),
            sender: self.sender.clone(),
            timestamp: self.timestamp,
            is_user: false,
        }
    }

    pub fn from_row(row: &Row) -> Result<ChatMessage, String> {
        let id: u32 = row.try_get(0).unwrap();
        let content: &str = row.try_get(1).unwrap();
        let ai_response: &str = row.try_get(2).unwrap();
        let sender: &str = row.try_get(3).unwrap();
        let timestamp: &str = row.try_get(4).unwrap();
        let timestamp: DateTime<Utc> = DateTime::from_str(timestamp).unwrap();
        let is_user: &str = row.try_get(5).unwrap();
        let is_user = is_user == "true";

        Ok(ChatMessage {
            id,
            content: content.to_string(),
            ai_response: ai_response.to_string(),
            sender: sender.to_string(),
            timestamp,
            is_user,
        })
    }
}

#[allow(clippy::await_holding_lock)]
pub async fn save_message(client: &DBClient, message: ChatMessage) {
    let client = client.lock().unwrap();

    info!("{message:?}");

    let statement = format!(
        r#"INSERT INTO chat_messages
    (id, content, ai_response, sender, "timestamp", is_user, created_at)
    VALUES({}, '{}', '{}', '{}', '{}', '{}', CURRENT_TIMESTAMP);"#,
        message.id,
        escape_sql_string(&message.content),
        escape_sql_string(&message.ai_response),
        message.sender,
        message.timestamp,
        message.is_user
    );

    let st = Statement::new(statement);

    let res = client.execute(st).await;
    drop(client);
    match res {
        Ok(s) => info!("{s:?}"),
        Err(e) => error!("{e}"),
    }
}

#[allow(clippy::await_holding_lock)]
pub async fn get_messages(client: &DBClient, user_id: &str) -> Vec<ChatMessage> {
    let client = client.lock().unwrap();

    let stmt = libsql_client::Statement::with_args(
        "
            SELECT id, content, ai_response, sender, timestamp, is_user, created_at
             FROM chat_messages
             WHERE sender = ?
             ORDER BY timestamp ASC
            ",
        &[user_id],
    );

    let res = client.execute(stmt).await.unwrap();

    drop(client);
    let rows: Vec<ChatMessage> = res
        .rows
        .iter()
        .filter_map(|r| ChatMessage::from_row(r).ok())
        .collect();
    rows
}
