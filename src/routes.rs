use actix_web::{HttpResponse, Responder, Result, delete, get, patch, post, web};
use chrono::{DateTime, Utc};
use libsql_client::Row;
use log::error;
use maud::{Markup, html};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::config::Server;
use crate::database::{self, DBClient};
use crate::llm;
use crate::view::render_item;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: u32,
    pub task: String,
    pub completed: bool,
}
impl Item {
    pub fn from_row(row: &Row) -> Result<Item, String> {
        let Ok(id) = row.try_get::<u32>(0) else {
            let err = format!("Item::from_row {row:?}");
            return Err(err);
        };

        let Ok(task) = row.try_get::<&str>(1) else {
            let err = format!("Item::from_row {row:?}");
            return Err(err);
        };

        let Ok(completed) = row.try_get::<&str>(2) else {
            let err = format!("Item::from_row {row:?}");
            return Err(err);
        };

        let completed: bool = completed.parse().unwrap();

        Ok(Item {
            id,
            task: task.to_string(),
            completed,
        })
    }
}

#[derive(Deserialize)]
pub struct CreateTodoRequest {
    pub task: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: u32,
    pub content: String,
    pub sender: String,
    pub timestamp: DateTime<Utc>,
    pub is_user: bool,
}

impl ChatMessage {
    pub fn from_row(row: &Row) -> Result<ChatMessage, String> {
        let id: u32 = row.try_get(0).unwrap();
        let content: &str = row.try_get(1).unwrap();
        let sendr: &str = row.try_get(2).unwrap();
        let timestamp: &str = row.try_get(3).unwrap();
        let timestamp: DateTime<Utc> = DateTime::from_str(timestamp).unwrap();
        let is_user: &str = row.try_get(4).unwrap();
        let is_user = is_user == "true";

        Ok(ChatMessage {
            id,
            content: content.to_string(),
            sender: sendr.to_string(),
            timestamp,
            is_user,
        })
    }
}

#[derive(Deserialize)]
pub struct SendMessageRequest {
    pub message: String,
}

#[post("/items/single")]
pub async fn create_item(
    form: web::Form<CreateTodoRequest>,
    client: web::Data<DBClient>,
) -> Result<Markup> {
    let client: &DBClient = client.get_ref();
    let id = random_id();

    let item = Item {
        id,
        task: form.task.clone(),
        completed: false,
    };
    database::create_item(client, item.clone()).await;

    Ok(render_item(&item))
}

#[patch("items/{id}/toggle")]
pub async fn toggle_item(path: web::Path<i64>, client: web::Data<DBClient>) -> Result<Markup> {
    let id = path.into_inner();
    let client: &DBClient = client.get_ref();
    let item = database::toggle_item(client, id).await;

    if let Ok(item) = item {
        Ok(render_item(&item))
    } else {
        Ok(html! { "" })
    }
}

#[delete("items/{id}")]
pub async fn delete_item(path: web::Path<i64>, client: web::Data<DBClient>) -> Result<Markup> {
    let id = path.into_inner();
    let client: &DBClient = client.get_ref();

    database::delete_item(client, id).await;
    Ok(html! { "" })
}

pub fn random_id() -> u32 {
    let mut rng = rand::rng();
    rng.random::<u32>()
}

#[post("chat")]
pub async fn send_message(
    form: web::Form<SendMessageRequest>,
    client: web::Data<DBClient>,
    config: web::Data<Server>,
) -> Result<Markup> {
    log::info!("Received chat message: {}", form.message);
    let x: &DBClient = client.get_ref();

    let chat_id = random_id();
    // Add user message
    let user_message = ChatMessage {
        id: chat_id,
        content: form.message.clone(),
        sender: "You".to_string(),
        timestamp: Utc::now(),
        is_user: true,
    };
    database::save_message(x, user_message.clone()).await;

    log::info!("Added user message with ID: {chat_id}");

    // Generate AI response
    let ai_response =
        generate_ai_response(&form.message, &config.nest_api(), &config.nest_api_key(), x).await;

    log::info!("Generated AI response: {ai_response}");

    let ai_message = ChatMessage {
        id: random_id(),
        content: ai_response,
        sender: "Assistant".to_string(),
        timestamp: Utc::now(),
        is_user: false,
    };
    database::save_message(x, ai_message.clone()).await;

    // Return both messages as HTML
    Ok(html! {
        (render_chat_message(&user_message))
        (render_chat_message(&ai_message))
    })
}

async fn generate_ai_response(
    user_message: &str,
    nest_api: &str,
    nest_api_key: &str,
    db_client: &DBClient,
) -> String {
    match llm::simple_chat(nest_api, nest_api_key, user_message, db_client).await {
        Ok(a) => a,
        Err(e) => {
            match e {
                llm::LlmError::Request(error) => error!("{error}"),
                llm::LlmError::Auth(error) => error!("{error}"),
                llm::LlmError::Parse(error) => error!("{error}"),
            };

            "Something went wrong contacting the agent".to_string()
        }
    }
}

fn render_chat_message(message: &ChatMessage) -> Markup {
    html! {
        div class={
            @if message.is_user {
                "chat chat-end"
            } @else {
                "chat chat-start"
            }
        } {
            div class="chat-image avatar" {
                div class="w-10 rounded-full" {
                    div class={
                        @if message.is_user {
                            "w-10 h-10 bg-secondary rounded-full flex items-center justify-center text-secondary-content font-bold"
                        } @else {
                            "w-10 h-10 bg-primary rounded-full flex items-center justify-center text-primary-content font-bold"
                        }
                    } {
                        @if message.is_user {
                            "Y"
                        } @else {
                            "A"
                        }
                    }
                }
            }
            div class="chat-header" {
                (message.sender)
                time class="text-xs opacity-50" {
                    (message.timestamp.format("%H:%M"))
                }
            }
            div class={
                @if message.is_user {
                    "chat-bubble chat-bubble-primary"
                } @else {
                    "chat-bubble"
                }
            } {
                (message.content)
            }
        }
    }
}

#[get("/healthz")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok()
}
