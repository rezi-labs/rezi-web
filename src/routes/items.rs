use actix_web::error::ParseError;

use actix_web::{HttpRequest, HttpResponse, Result, delete, get, patch, post, web};
use log::info;
use serde::Deserialize;

use crate::database::{self, DBClient};
use crate::view::{self, render_item};

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
) -> Result<HttpResponse> {
    let client: &DBClient = client.get_ref();
    let user = match super::get_user_or_redirect(&req) {
        Ok(user) => user,
        Err(response) => return Ok(response),
    };

    let item = database::items::Item {
        id: None,
        owner_id: user.id().to_string(),
        task: form.task.clone(),
        completed: 0,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    let res = database::items::create_item(client, item.clone()).await;

    let Ok(item) = res else {
        return Ok(HttpResponse::InternalServerError()
            .content_type("text/html; charset=utf-8")
            .body(""));
    };

    let markup = render_item(&item);
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(markup.into_string()))
}

#[patch("items/{id}/toggle")]
pub async fn toggle_item(
    path: web::Path<i64>,
    client: web::Data<DBClient>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let id = path.into_inner();
    let user = match super::get_user_or_redirect(&req) {
        Ok(user) => user,
        Err(response) => return Ok(response),
    };

    info!("toggle_item: {id}");
    let client: &DBClient = client.get_ref();
    let item = database::items::toggle_item(client, id, user.id().to_string()).await;

    if let Ok(item) = item {
        let markup = render_item(&item);
        Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(markup.into_string()))
    } else {
        Ok(HttpResponse::InternalServerError()
            .content_type("text/html; charset=utf-8")
            .body(""))
    }
}

#[delete("items/{id}")]
pub async fn delete_item(
    path: web::Path<i64>,
    client: web::Data<DBClient>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let id = path.into_inner();
    let client: &DBClient = client.get_ref();
    let user = match super::get_user_or_redirect(&req) {
        Ok(user) => user,
        Err(response) => return Ok(response),
    };

    database::items::delete_item(client, id, user.id().to_owned()).await;
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(""))
}

#[patch("items/{id}")]
pub async fn update_item(
    path: web::Path<i64>,
    form: web::Form<UpdateTodoRequest>,
    client: web::Data<DBClient>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let id = path.into_inner();
    let user = match super::get_user_or_redirect(&req) {
        Ok(user) => user,
        Err(response) => return Ok(response),
    };
    let client: &DBClient = client.get_ref();

    info!("update_item: {id} with task: {}", form.task);

    let item =
        database::items::update_item(client, id, form.task.clone(), user.id().to_string()).await;

    match item {
        Ok(item) => {
            let markup = render_item(&item);
            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(markup.into_string()))
        }
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
) -> Result<HttpResponse> {
    let id = path.into_inner();
    let user = match super::get_user_or_redirect(&req) {
        Ok(user) => user,
        Err(response) => return Ok(response),
    };
    let client: &DBClient = client.get_ref();

    let item = database::items::get_item(client, id, user.id().to_string()).await;

    if let Ok(item) = item {
        let markup = view::items::render_item_edit(&item);
        Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(markup.into_string()))
    } else {
        Ok(HttpResponse::InternalServerError()
            .content_type("text/html; charset=utf-8")
            .body(""))
    }
}

#[get("items/{id}/cancel")]
pub async fn cancel_edit_item(
    path: web::Path<i64>,
    client: web::Data<DBClient>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let id = path.into_inner();
    let user = match super::get_user_or_redirect(&req) {
        Ok(user) => user,
        Err(response) => return Ok(response),
    };
    let client: &DBClient = client.get_ref();

    let item = database::items::get_item(client, id, user.id().to_string()).await;

    if let Ok(item) = item {
        let markup = render_item(&item);
        Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(markup.into_string()))
    } else {
        Ok(HttpResponse::InternalServerError()
            .content_type("text/html; charset=utf-8")
            .body(""))
    }
}


