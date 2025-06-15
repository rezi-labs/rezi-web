use actix_web::{HttpResponse, Responder, Result, delete, get, patch, post, web};
use chrono::{DateTime, Utc};
use log::error;
use maud::{Markup, html};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::llm;
use crate::view::render_item;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: u32,
    pub task: String,
    pub completed: bool,
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

#[derive(Deserialize)]
pub struct SendMessageRequest {
    pub message: String,
}

// Simple in-memory storage for demo purposes
lazy_static::lazy_static! {
    pub static ref ITEM_STORAGE: Arc<Mutex<HashMap<u32, Item>>> = Arc::new(Mutex::new(HashMap::new()));
    pub static ref CHAT_STORAGE: Arc<Mutex<Vec<ChatMessage>>> = Arc::new(Mutex::new(Vec::new()));
    pub static ref NEXT_ID: Arc<Mutex<u32>> = Arc::new(Mutex::new(1));
    static ref NEXT_CHAT_ID: Arc<Mutex<u32>> = Arc::new(Mutex::new(1));
}

pub fn initialize_sample_data() {
    let mut storage = ITEM_STORAGE.lock().unwrap();
    let mut next_id = NEXT_ID.lock().unwrap();

    // Only initialize if storage is empty
    if storage.is_empty() {
        storage.insert(
            1,
            Item {
                id: 1,
                task: "Sample todo item".to_string(),
                completed: false,
            },
        );

        storage.insert(
            2,
            Item {
                id: 2,
                task: "Completed todo item".to_string(),
                completed: true,
            },
        );

        *next_id = 3;
    }

    // Initialize sample chat messages
    let mut chat_storage = CHAT_STORAGE.lock().unwrap();
    let mut next_chat_id = NEXT_CHAT_ID.lock().unwrap();

    if chat_storage.is_empty() {
        chat_storage.push(ChatMessage {
            id: 1,
            content: "Hello! How's your day going?".to_string(),
            sender: "John".to_string(),
            timestamp: Utc::now(),
            is_user: false,
        });

        chat_storage.push(ChatMessage {
            id: 2,
            content: "Pretty good! Just working on some todos.".to_string(),
            sender: "You".to_string(),
            timestamp: Utc::now(),
            is_user: true,
        });

        *next_chat_id = 3;
    }
}

#[post("")]
async fn create_item(form: web::Form<CreateTodoRequest>) -> Result<Markup> {
    let mut storage = ITEM_STORAGE.lock().unwrap();
    let mut next_id = NEXT_ID.lock().unwrap();

    let todo_item = Item {
        id: *next_id,
        task: form.task.clone(),
        completed: false,
    };

    storage.insert(*next_id, todo_item.clone());
    *next_id += 1;

    Ok(render_item(&todo_item))
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

fn random_chat_id() -> u32 {
    let mut rng = rand::rng();
    rng.random::<u32>()
}

#[post("")]
async fn send_message(form: web::Form<SendMessageRequest>) -> Result<Markup> {
    log::info!("Received chat message: {}", form.message);

    let mut chat_storage = CHAT_STORAGE.lock().unwrap();

    let chat_id = random_chat_id();
    // Add user message
    let user_message = ChatMessage {
        id: chat_id,
        content: form.message.clone(),
        sender: "You".to_string(),
        timestamp: Utc::now(),
        is_user: true,
    };
    chat_storage.push(user_message.clone());
    log::info!("Added user message with ID: {}", chat_id);

    // Drop the locks before async call
    drop(chat_storage);

    // Generate AI response
    let ai_response = generate_ai_response(&form.message).await;
    log::info!("Generated AI response: {}", ai_response);

    // Re-acquire locks for AI message
    let mut chat_storage = CHAT_STORAGE.lock().unwrap();
    let mut next_chat_id = NEXT_CHAT_ID.lock().unwrap();

    let ai_message = ChatMessage {
        id: *next_chat_id,
        content: ai_response,
        sender: "Assistant".to_string(),
        timestamp: Utc::now(),
        is_user: false,
    };
    chat_storage.push(ai_message.clone());
    log::info!("Added AI message with ID: {}", *next_chat_id);
    *next_chat_id += 1;

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
