use actix_web::{HttpResponse, Responder, Result, delete, get, patch, post, web};
use chrono::{DateTime, Utc};
use maud::{Markup, html};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::view::render_todo_item;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoItem {
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
    pub static ref TODO_STORAGE: Arc<Mutex<HashMap<u32, TodoItem>>> = Arc::new(Mutex::new(HashMap::new()));
    pub static ref CHAT_STORAGE: Arc<Mutex<Vec<ChatMessage>>> = Arc::new(Mutex::new(Vec::new()));
    static ref NEXT_ID: Arc<Mutex<u32>> = Arc::new(Mutex::new(1));
    static ref NEXT_CHAT_ID: Arc<Mutex<u32>> = Arc::new(Mutex::new(1));
}

pub fn initialize_sample_data() {
    let mut storage = TODO_STORAGE.lock().unwrap();
    let mut next_id = NEXT_ID.lock().unwrap();

    // Only initialize if storage is empty
    if storage.is_empty() {
        storage.insert(
            1,
            TodoItem {
                id: 1,
                task: "Sample todo item".to_string(),
                completed: false,
            },
        );

        storage.insert(
            2,
            TodoItem {
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
async fn create_todo(form: web::Form<CreateTodoRequest>) -> Result<Markup> {
    let mut storage = TODO_STORAGE.lock().unwrap();
    let mut next_id = NEXT_ID.lock().unwrap();

    let todo_item = TodoItem {
        id: *next_id,
        task: form.task.clone(),
        completed: false,
    };

    storage.insert(*next_id, todo_item.clone());
    *next_id += 1;

    Ok(render_todo_item(&todo_item))
}

#[patch("/{id}/toggle")]
async fn toggle_todo(path: web::Path<u32>) -> Result<Markup> {
    let id = path.into_inner();
    let mut storage = TODO_STORAGE.lock().unwrap();

    if let Some(todo) = storage.get_mut(&id) {
        todo.completed = !todo.completed;
        log::info!("{:?}", todo);
        Ok(render_todo_item(todo))
    } else {
        Ok(html! { "" })
    }
}

#[delete("/{id}")]
async fn delete_todo(path: web::Path<u32>) -> Result<Markup> {
    let id = path.into_inner();
    let mut storage = TODO_STORAGE.lock().unwrap();

    storage.remove(&id);
    Ok(html! { "" })
}

pub fn todo_scope() -> actix_web::Scope {
    web::scope("/todos")
        .service(create_todo)
        .service(toggle_todo)
        .service(delete_todo)
}

#[post("")]
async fn send_message(form: web::Form<SendMessageRequest>) -> Result<Markup> {
    log::info!("Received chat message: {}", form.message);

    let mut chat_storage = CHAT_STORAGE.lock().unwrap();
    let mut next_chat_id = NEXT_CHAT_ID.lock().unwrap();

    // Add user message
    let user_message = ChatMessage {
        id: *next_chat_id,
        content: form.message.clone(),
        sender: "You".to_string(),
        timestamp: Utc::now(),
        is_user: true,
    };
    chat_storage.push(user_message.clone());
    log::info!("Added user message with ID: {}", *next_chat_id);
    *next_chat_id += 1;

    // Drop the locks before async call
    drop(chat_storage);
    drop(next_chat_id);

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
    match crate::llm::simple_chat(user_message).await {
        Ok(response) => response,
        Err(e) => {
            log::error!("LLM API error: {:?}", e);
            // Fallback to simple responses
            let message_lower = user_message.to_lowercase();
            if message_lower.contains("hello") || message_lower.contains("hi") {
                "Hello! How can I help you today?".to_string()
            } else if message_lower.contains("todo") || message_lower.contains("task") {
                "I see you're talking about todos! You can manage your tasks using the todo list feature.".to_string()
            } else if message_lower.contains("how are you") {
                "I'm doing great! Thanks for asking. How are you doing?".to_string()
            } else if message_lower.contains("help") {
                "I'm here to help! You can ask me about todos, chat with me, or just have a conversation.".to_string()
            } else {
                "That's interesting! Tell me more about it.".to_string()
            }
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
