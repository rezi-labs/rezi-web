use actix_web::error::ParseError;
use actix_web::http::header::CONTENT_DISPOSITION;
use actix_web::{HttpRequest, HttpResponse, Result, delete, get, patch, post, web};
use chrono::Utc;
use log::info;
use maud::{Markup, html};
use serde::Deserialize;

use crate::config::Server;
use crate::csv;
use crate::database::{self, DBClient};
use crate::view::{self, message, render_item};

#[derive(Deserialize)]
pub struct CreateTodoRequest {
    pub task: String,
}

#[derive(Deserialize)]
pub struct UpdateTodoRequest {
    pub task: String,
}

#[post("/items/single")]
pub async fn create_item(
    form: web::Form<CreateTodoRequest>,
    client: web::Data<DBClient>,
    req: HttpRequest,
) -> Result<Markup> {
    let client: &DBClient = client.get_ref();
    let user = super::get_user(req).unwrap();

    let item = database::items::Item {
        id: None,
        owner_id: user.id().to_string(),
        task: form.task.clone(),
        completed: 0,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    let res = database::items::create_item(client, item.clone()).await;

    let Ok(item) = res else { return Ok(html!()) };

    Ok(render_item(&item))
}

#[patch("items/{id}/toggle")]
pub async fn toggle_item(
    path: web::Path<i64>,
    client: web::Data<DBClient>,
    req: HttpRequest,
) -> Result<Markup> {
    let id = path.into_inner();
    let user = super::get_user(req).unwrap();

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
    let user = super::get_user(req).unwrap();

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
    let user = super::get_user(req).unwrap();
    let client: &DBClient = client.get_ref();

    info!("update_item: {id} with task: {}", form.task);

    let item =
        database::items::update_item(client, id, form.task.clone(), user.id().to_string()).await;

    match item {
        Ok(item) => Ok(render_item(&item)),
        Err(err) => {
            log::error!("{err}");
            Err(ParseError::Incomplete.into())
        }
    }
}

#[get("items/{id}/edit")]
pub async fn edit_item(
    path: web::Path<i64>,
    client: web::Data<DBClient>,
    req: HttpRequest,
) -> Result<Markup> {
    let id = path.into_inner();
    let user = super::get_user(req).unwrap();
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
    let user = super::get_user(req).unwrap();
    let client: &DBClient = client.get_ref();

    let item = database::items::get_item(client, id, user.id().to_string()).await;

    if let Ok(item) = item {
        Ok(render_item(&item))
    } else {
        Ok(html! { "" })
    }
}

#[post("/ai/items")]
pub async fn create_item_with_ai(
    form: web::Form<super::messages::SendMessageRequest>,
    client: web::Data<DBClient>,
    config: web::Data<Server>,
    req: HttpRequest,
) -> Result<Markup> {
    let user = super::get_user(req).unwrap();
    // delay if delay is on
    if config.delay() {
        tokio::time::sleep(std::time::Duration::from_millis(2000)).await;
    }

    log::info!("Received task message: {}", form.message);
    let db_client: &DBClient = client.get_ref();

    // Generate AI response
    let ai_response = super::generate_task_response(
        &form.message,
        &config.nest_api(),
        &config.nest_api_key(),
        db_client,
        user.id().to_string(),
    )
    .await;

    let user_message = database::messages::ChatMessage {
        id: None,
        content: form.message.clone(),
        ai_response: ai_response.clone(),
        owner_id: user.id().to_string(),
        created_at: Utc::now(),
        is_user: 1,
    };
    let Ok(message) = database::messages::save_message(db_client, user_message.clone()).await
    else {
        return Err(ParseError::Incomplete.into());
    };

    // do not save ai message
    let ai_message = database::messages::ChatMessage {
        id: None,
        content: ai_response.clone(),
        ai_response: ai_response.clone(),
        owner_id: "Agent".to_string(),
        created_at: Utc::now(),
        is_user: 0,
    };

    Ok(html! {
        (view::message::render(&message, Some(user.to_owned())))
        (message::render(&ai_message, None))
    })
}

#[get("/items/csv")]
pub async fn items_csv(client: web::Data<DBClient>, req: HttpRequest) -> Result<HttpResponse> {
    let user = super::get_user(req).unwrap();
    let owner_id = user.id().to_string();
    let db_client: &DBClient = client.get_ref();
    let items = database::items::get_items(db_client, owner_id)
        .await
        .unwrap_or_default();
    let csv_file = csv::items_to_events(items.as_slice());

    let response = HttpResponse::Ok()
        .append_header((CONTENT_DISPOSITION, "attachment; filename=\"calendar.csv\""))
        .content_type("application/octet-stream")
        .body(csv_file);

    Ok(response)
}
