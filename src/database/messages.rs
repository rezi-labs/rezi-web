use libsql_orm::Filter;
use libsql_orm::FilterOperator;
use libsql_orm::Model;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::database::DBClient;

#[derive(Model, Debug, Clone, Serialize, Deserialize)]
#[table_name("messages")]
pub struct ChatMessage {
    pub id: std::option::Option<i64>,
    pub content: String,
    pub ai_response: String,
    pub owner_id: String,
    pub created_at: DateTime<Utc>,
    pub is_user: u16,
    pub reply_to_id: std::option::Option<i64>,
}

impl ChatMessage {
    pub fn id(&self) -> String {
        let Some(id) = self.id else {
            log::error!("no id: {self:?}");
            return "none".to_string();
        };
        id.to_string()
    }

    pub fn is_user(&self) -> bool {
        self.is_user == 1
    }
}

pub async fn save_message(client: &DBClient, message: ChatMessage) -> Result<ChatMessage, String> {
    let db = super::unlock_client(client).await;
    let res = message.create(&db).await;
    drop(db);

    match res {
        Ok(m) => {
            log::info!("Created message: {}", m.id());
            Ok(m)
        }
        Err(err) => {
            log::error!("Failed to save message: {err}");
            Err(err.to_string())
        }
    }
}

pub async fn get_message_by_id(client: &DBClient, id: i64, user_id: &str) -> Option<ChatMessage> {
    let db = super::unlock_client(client).await;
    let result = ChatMessage::find_by_id(id, &db).await;
    drop(db);

    match result {
        Ok(message) => {
            log::info!("Found message with id: {id}");
            if message.clone().unwrap().owner_id != user_id {
                log::info!("Not allowed access to message with id: {id}");
                return None;
            }
            message
        }
        Err(err) => {
            log::error!("Failed to get message by id {id}: {err}");
            None
        }
    }
}

pub async fn get_messages(client: &DBClient, owner_id: &str) -> Vec<ChatMessage> {
    let db = super::unlock_client(client).await;
    let result = ChatMessage::find_where(
        FilterOperator::Single(Filter::eq("owner_id".to_string(), owner_id)),
        &db,
    )
    .await;
    drop(db);

    match result {
        Ok(messages) => {
            log::info!("Found {} messages for owner: {owner_id}", messages.len());
            messages
        }
        Err(err) => {
            log::error!("Failed to get messages for {owner_id}: {err}");
            Vec::new()
        }
    }
}
