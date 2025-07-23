use actix_web::http::header::CONTENT_DISPOSITION;
use actix_web::web::Data;
use actix_web::{
    HttpMessage, HttpRequest, HttpResponse, Responder, Result, delete, get, patch, post, web,
};
use chrono::{DateTime, Utc};
use libsql_client::Row;
use log::{error, info};
use maud::{Markup, html};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use url::Url;

use crate::config::Server;
use crate::database::items::int_to_bool;
use crate::database::{self, DBClient};
use crate::view::{self, render_item};
use crate::{csv, ical, llm, message, unsafe_token_decode, witch};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: u32,
    pub task: String,
    pub completed: bool,
}
impl Item {
    pub fn random_item() -> Item {
        Item {
            id: random_id(),
            task: String::from("Random Task"),
            completed: false,
        }
    }

    pub fn from_row(row: &Row) -> Result<Item, String> {
        let Ok(id) = row.try_get::<u32>(0) else {
            let err = format!("Item::from_row {row:?}");
            return Err(err);
        };

        let Ok(task) = row.try_get::<&str>(1) else {
            let err = format!("Item::from_row {row:?}");
            return Err(err);
        };

        let Ok(completed) = row.try_get::<i64>(2) else {
            let err = format!("Item::from_row {row:?}");
            return Err(err);
        };

        let completed: bool = int_to_bool(completed);

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

#[derive(Deserialize)]
pub struct UpdateTodoRequest {
    pub task: String,
}

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

#[derive(Deserialize)]
pub struct SendMessageRequest {
    pub message: String,
}

pub fn get_user(req: HttpRequest) -> Option<unsafe_token_decode::User> {
    req.extensions()
        .get::<Data<unsafe_token_decode::User>>()
        .map(|u| u.as_ref().clone())
}

#[post("/items/single")]
pub async fn create_item(
    form: web::Form<CreateTodoRequest>,
    client: web::Data<DBClient>,
    req: HttpRequest,
) -> Result<Markup> {
    let client: &DBClient = client.get_ref();
    let id = random_id();
    let user = get_user(req).unwrap();

    let item = Item {
        id,
        task: form.task.clone(),
        completed: false,
    };
    database::items::create_item(client, item.clone(), user.id().to_string()).await;

    Ok(render_item(&item))
}

#[patch("items/{id}/toggle")]
pub async fn toggle_item(
    path: web::Path<i64>,
    client: web::Data<DBClient>,
    req: HttpRequest,
) -> Result<Markup> {
    let id = path.into_inner();
    let user = get_user(req).unwrap();

    info!("toggle_item: {id}");
    let client: &DBClient = client.get_ref();
    let item = database::items::toggle_item(client, id, user.id().to_string()).await;

    if let Ok(item) = item {
        Ok(render_item(&item))
    } else {
        Ok(html! { "" })
    }
}

#[delete("items/{id}")]
pub async fn delete_item(
    path: web::Path<i64>,
    client: web::Data<DBClient>,
    req: HttpRequest,
) -> Result<Markup> {
    let id = path.into_inner();
    let client: &DBClient = client.get_ref();
    let user = get_user(req).unwrap();

    database::items::delete_item(client, id, user.id().to_owned()).await;
    Ok(html! { "" })
}

#[patch("items/{id}")]
pub async fn update_item(
    path: web::Path<i64>,
    form: web::Form<UpdateTodoRequest>,
    client: web::Data<DBClient>,
    req: HttpRequest,
) -> Result<Markup> {
    let id = path.into_inner();
    let user = get_user(req).unwrap();
    let client: &DBClient = client.get_ref();

    info!("update_item: {id} with task: {}", form.task);

    let item =
        database::items::update_item(client, id, form.task.clone(), user.id().to_string()).await;

    if let Ok(item) = item {
        Ok(render_item(&item))
    } else {
        Ok(html! { "" })
    }
}

#[get("items/{id}/edit")]
pub async fn edit_item(
    path: web::Path<i64>,
    client: web::Data<DBClient>,
    req: HttpRequest,
) -> Result<Markup> {
    let id = path.into_inner();
    let user = get_user(req).unwrap();
    let client: &DBClient = client.get_ref();

    let item = database::items::get_item(client, id, user.id().to_string()).await;

    if let Ok(item) = item {
        Ok(view::items::render_item_edit(&item))
    } else {
        Ok(html! { "" })
    }
}

#[get("items/{id}/cancel")]
pub async fn cancel_edit_item(
    path: web::Path<i64>,
    client: web::Data<DBClient>,
    req: HttpRequest,
) -> Result<Markup> {
    let id = path.into_inner();
    let user = get_user(req).unwrap();
    let client: &DBClient = client.get_ref();

    let item = database::items::get_item(client, id, user.id().to_string()).await;

    if let Ok(item) = item {
        Ok(render_item(&item))
    } else {
        Ok(html! { "" })
    }
}

pub fn random_id() -> u32 {
    let mut rng = rand::rng();
    rng.random::<u32>()
}

#[post("/ai/items")]
pub async fn create_item_with_ai(
    form: web::Form<SendMessageRequest>,
    client: web::Data<DBClient>,
    config: web::Data<Server>,
    req: HttpRequest,
) -> Result<Markup> {
    let user = get_user(req).unwrap();
    // delay if delay is on
    if config.delay() {
        tokio::time::sleep(std::time::Duration::from_millis(2000)).await;
    }

    log::info!("Received task message: {}", form.message);
    let db_client: &DBClient = client.get_ref();

    // Generate AI response
    let ai_response = generate_task_response(
        &form.message,
        &config.nest_api(),
        &config.nest_api_key(),
        db_client,
        user.id().to_string(),
    )
    .await;

    let chat_id = random_id();

    let user_message = ChatMessage {
        id: chat_id,
        content: form.message.clone(),
        ai_response: ai_response.clone(),
        sender: user.id().to_string(),
        timestamp: Utc::now(),
        is_user: true,
    };
    database::messages::save_message(db_client, user_message.clone()).await;

    // do not save ai message
    let ai_message = ChatMessage {
        id: chat_id,
        content: ai_response.clone(),
        ai_response: ai_response.clone(),
        sender: "Agent".to_string(),
        timestamp: Utc::now(),
        is_user: false,
    };

    Ok(html! {
        (message::render(&user_message, Some(user.to_owned())))
        (message::render(&ai_message, None))
    })
}

#[post("chat")]
pub async fn send_message(
    form: web::Form<SendMessageRequest>,
    client: web::Data<DBClient>,
    config: web::Data<Server>,
    req: HttpRequest,
) -> Result<Markup> {
    let user = get_user(req).unwrap();

    log::info!("Received chat message: {}", form.message);
    let db_client: &DBClient = client.get_ref();
    // delay if delay is on
    if config.delay() {
        tokio::time::sleep(std::time::Duration::from_millis(2000)).await;
    }

    let message = form.message.clone();

    let url = Url::parse(&message);
    let ai_response = match url {
        Ok(url) => {
            let url = url.as_str().to_string();

            let hex = witch::hex(url).await;

            let Ok(hex) = hex else {
                return Err(actix_web::error::ErrorBadRequest(
                    "Could not fetch hex".to_string(),
                ));
            };

            generate_task_response(
                &hex,
                &config.nest_api(),
                &config.nest_api_key(),
                db_client,
                user.id().to_string(),
            )
            .await
        }
        Err(_) => {
            generate_ai_response(&form.message, &config.nest_api(), &config.nest_api_key()).await
        }
    };

    let chat_id = random_id();

    let user_message = ChatMessage {
        id: chat_id,
        content: form.message.clone(),
        ai_response: ai_response.clone(),
        sender: user.id().to_string(),
        timestamp: Utc::now(),
        is_user: true,
    };
    database::messages::save_message(db_client, user_message.clone()).await;

    // do not save ai message
    let ai_message = ChatMessage {
        id: chat_id,
        content: ai_response.clone(),
        ai_response: ai_response.clone(),
        sender: "Agent".to_string(),
        timestamp: Utc::now(),
        is_user: false,
    };

    // Return both messages as HTML
    Ok(html! {
        (message::render(&user_message, Some(user.to_owned())))
        (message::render(&ai_message, None))
    })
}

async fn generate_ai_response(user_message: &str, nest_api: &str, nest_api_key: &str) -> String {
    match llm::simple_chat_response(nest_api, nest_api_key, user_message).await {
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

async fn generate_task_response(
    user_message: &str,
    nest_api: &str,
    nest_api_key: &str,
    db_client: &DBClient,
    user_id: String,
) -> String {
    match llm::simple_item_response(nest_api, nest_api_key, user_message, user_id, db_client).await
    {
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

#[get("/items/ical")]
pub async fn items_ical(client: web::Data<DBClient>, req: HttpRequest) -> Result<HttpResponse> {
    let user = get_user(req).unwrap();
    let owner_id = user.id().to_string();
    let db_client: &DBClient = client.get_ref();
    let items = database::items::get_items(db_client, owner_id).await;
    let ical_file = ical::items_to_events(items.as_slice());

    let response = HttpResponse::Ok()
        .append_header((CONTENT_DISPOSITION, "attachment; filename=\"calendar.ics\""))
        .content_type("text/calendar")
        .body(ical_file);

    Ok(response)
}

#[get("/items/csv")]
pub async fn items_csv(client: web::Data<DBClient>, req: HttpRequest) -> Result<HttpResponse> {
    let user = get_user(req).unwrap();
    let owner_id = user.id().to_string();
    let db_client: &DBClient = client.get_ref();
    let items = database::items::get_items(db_client, owner_id).await;
    let csv_file = csv::items_to_events(items.as_slice());

    let response = HttpResponse::Ok()
        .append_header((CONTENT_DISPOSITION, "attachment; filename=\"calendar.csv\""))
        .content_type("application/octet-stream")
        .body(csv_file);

    Ok(response)
}

#[get("/healthz")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok()
}
