use libsql_orm::Filter;
use libsql_orm::FilterOperator;
use libsql_orm::Model;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::database::DBClient;
use crate::routes::random_id;

#[derive(Model, Debug, Clone, Serialize, Deserialize)]
#[table_name("messages")]
pub struct ChatMessage {
    pub id: std::option::Option<i64>,
    pub content: String,
    pub ai_response: String,
    pub owner_id: String,
    pub created_at: DateTime<Utc>,
    pub is_user: bool,
}

impl ChatMessage {
    pub fn ai_message(&self) -> Self {
        ChatMessage {
            id: Some(self.id.unwrap_or(random_id())),
            content: self.ai_response.clone(),
            ai_response: self.ai_response.clone(),
            owner_id: self.owner_id.clone(),
            created_at: self.created_at,
            is_user: false,
        }
    }

    pub fn id(&self) -> String {
        let Some(id) = self.id else {
            log::error!("no id: {self:?}");
            return "none".to_string();
        };
        id.to_string()
    }

    pub fn is_user(&self) -> bool {
        self.is_user
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
