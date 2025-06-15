use actix_web::{HttpResponse, Responder, Result, delete, get, patch, post, web};
use chrono::{DateTime, Utc};
use libsql_client::Row;
use log::error;
use maud::{Markup, html};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

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
        let id: u32 = row.try_get(0).unwrap();
        let task: &str = row.try_get(1).unwrap();
        let completed: u32 = row.try_get(4).unwrap();
        let completed = completed == 1;

        Ok(Item {
            id,
            task: task.to_string(),
            completed: completed,
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
            timestamp: timestamp,
            is_user: is_user,
        })
    }
}

#[derive(Deserialize)]
pub struct SendMessageRequest {
    pub message: String,
}


#[post("")]
async fn create_item(form: web::Form<CreateTodoRequest>, client: web::Data<DBClient>) -> Result<Markup> {

    let client: &DBClient = client.get_ref();
    let id= random_id();
    
    let item = Item {
        id: id,
        task: form.task.clone(),
        completed: false,
    };
    database::create_item(client, item.clone());
    

    Ok(render_item(&item))
}

#[patch("/{id}/toggle")]
async fn toggle_item(path: web::Path<u32>) -> Result<Markup> {
    let id = path.into_inner();
    let mut storage = ITEM_STORAGE.lock().unwrap();

    if let Some(todo) = storage.get_mut(&id) {
        todo.completed = !todo.completed;
        log::info!("{:?}", todo);
        Ok(render_item(todo))
    } else {
        Ok(html! { "" })
    }
}

#[delete("/{id}")]
async fn delete_item(path: web::Path<u32>) -> Result<Markup> {
    let id = path.into_inner();
    let mut storage = ITEM_STORAGE.lock().unwrap();

    storage.remove(&id);
    Ok(html! { "" })
}

pub fn item_scope() -> actix_web::Scope {
    web::scope("/items")
        .service(create_item)
        .service(toggle_item)
        .service(delete_item)
}

pub fn random_id() -> u32 {
    let mut rng = rand::rng();
    rng.random::<u32>()
}

#[post("")]
async fn send_message(
    form: web::Form<SendMessageRequest>,
    client: web::Data<DBClient>,
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

    log::info!("Added user message with ID: {}", chat_id);

    // Generate AI response
    let ai_response = generate_ai_response(&form.message).await;
    log::info!("Generated AI response: {}", ai_response);

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

async fn generate_ai_response(user_message: &str) -> String {
    match llm::simple_chat(user_message).await {
        Ok(a) => a,
        Err(e) => {
            match e {
                llm::LlmError::Request(error) => error!("{}", error),
                llm::LlmError::Auth(error) => error!("{}", error),
                llm::LlmError::Parse(error) => error!("{}", error),
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

pub fn chat_scope() -> actix_web::Scope {
    web::scope("/chat").service(send_message)
}

#[get("/healthz")]
async fn health() -> impl Responder {
    HttpResponse::Ok()
}
