use libsql_orm::Filter;
use libsql_orm::FilterOperator;
use libsql_orm::Model;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::database::DBClient2;
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
            log::error!("no id: {:?}", self);
            return "none".to_string();
        };
        id.to_string()
    }

    pub fn is_user(&self) -> bool {
        self.is_user
    }
}

#[allow(clippy::await_holding_lock)]
pub async fn save_message(client: &DBClient2, message: ChatMessage) -> Result<ChatMessage, String> {
    let db = client.lock().unwrap();

    let res = message.create(&db).await;
    match res {
        Ok(m) => {
            log::info!("{m:?}");
            return Ok(m);
        }
        Err(err) => {
            log::error!("{err}");
            Err(err.to_string())
        }
    }
}

#[allow(clippy::await_holding_lock)]
pub async fn get_messages(client: &DBClient2, owner_id: &str) -> Vec<ChatMessage> {
    let db = client.lock().unwrap();

    ChatMessage::find_where(
        FilterOperator::Single(Filter::eq("owner_id".to_string(), owner_id)),
        &db,
    )
    .await
    .unwrap()
}
