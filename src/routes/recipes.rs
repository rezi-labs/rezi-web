use actix_web::error::ParseError;
use actix_web::{HttpRequest, HttpResponse, Result, delete, get, patch, post, web};
use log::info;
use maud::{Markup, html};
use serde::Deserialize;

use crate::config::Server;
use crate::database::recipes::Recipe;
use crate::database::{self, DBClient};
use crate::routes::get_user;
use crate::view::{self, index};

#[derive(Deserialize)]
pub struct CreateRecipeRequest {
    pub title: Option<String>,
    pub url: Option<String>,
    pub content: String,
}

#[derive(Deserialize)]
pub struct UpdateRecipeRequest {
    pub title: Option<String>,
    pub url: Option<String>,
    pub content: Option<String>,
}

#[get("/recipes")]
pub async fn recipe_endpoint(
    server: web::Data<Server>,
    client: web::Data<DBClient>,
    req: HttpRequest,
) -> Result<Markup> {
    let user = get_user(req).unwrap();
    let client = client.get_ref();

    let recipes = database::recipes::get_recipes(client, user.id().to_string())
        .await
        .unwrap_or_default();

    let should_poll_reload = server.db_token().is_none();

    Ok(index(
        Some(view::recipes::recipes(recipes)),
        should_poll_reload,
        Some(&user),
    ))
}

#[post("/recipes")]
pub async fn create_recipe(
    form: web::Form<CreateRecipeRequest>,
    client: web::Data<DBClient>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let client: &DBClient = client.get_ref();
    let user = match crate::routes::get_user_or_redirect(&req) {
        Ok(user) => user,
        Err(response) => return Ok(response),
    };

    let recipe = Recipe {
        id: None,
        owner_id: user.id().to_string(),
        title: form.title.clone(),
        url: form.url.clone(),
        content: form.content.clone(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    let res = database::recipes::create_recipe(client, recipe).await;

    let Ok(recipe) = res else {
        let markup = html! {
            div class="alert alert-error" {
                "Failed to create recipe"
            }
        };
        return Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(markup.into_string()));
    };

    let markup = view::recipes::recipe_row(&recipe);
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(markup.into_string()))
}

#[get("/recipes/{id}")]
pub async fn get_recipe(
    path: web::Path<i64>,
    client: web::Data<DBClient>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let id = path.into_inner();
    let user = match crate::routes::get_user_or_redirect(&req) {
        Ok(user) => user,
        Err(response) => return Ok(response),
    };
    let client: &DBClient = client.get_ref();

    let recipe = database::recipes::get_recipe(client, id, user.id().to_string()).await;

    match recipe {
        Ok(recipe) => {
            let markup = view::recipes::recipe_row(&recipe);
            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(markup.into_string()))
        }
        Err(_) => Ok(HttpResponse::InternalServerError()
            .content_type("text/html; charset=utf-8")
            .body("")),
    }
}

#[patch("/recipes/{id}")]
pub async fn update_recipe(
    path: web::Path<i64>,
    form: web::Form<UpdateRecipeRequest>,
    client: web::Data<DBClient>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let id = path.into_inner();
    let user = match crate::routes::get_user_or_redirect(&req) {
        Ok(user) => user,
        Err(response) => return Ok(response),
    };
    let client: &DBClient = client.get_ref();

    info!("update_recipe: {id}");

    let recipe = database::recipes::update_recipe(
        client,
        id,
        form.title.clone(),
        form.url.clone(),
        form.content.clone(),
        user.id().to_string(),
    )
    .await;

    match recipe {
        Ok(recipe) => {
            let markup = view::recipes::recipe_row(&recipe);
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

#[delete("/recipes/{id}")]
pub async fn delete_recipe(
    path: web::Path<i64>,
    client: web::Data<DBClient>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let id = path.into_inner();
    let client: &DBClient = client.get_ref();
    let user = match crate::routes::get_user_or_redirect(&req) {
        Ok(user) => user,
        Err(response) => return Ok(response),
    };

    let _ = database::recipes::delete_recipe(client, id, user.id().to_string()).await;
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(""))
}

#[get("/recipes/{id}/edit")]
pub async fn edit_recipe(
    path: web::Path<i64>,
    client: web::Data<DBClient>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let id = path.into_inner();
    let user = match crate::routes::get_user_or_redirect(&req) {
        Ok(user) => user,
        Err(response) => return Ok(response),
    };
    let client: &DBClient = client.get_ref();

    let recipe = database::recipes::get_recipe(client, id, user.id().to_string()).await;

    if let Ok(recipe) = recipe {
        let markup = view::recipes::recipe_edit_row(&recipe);
        Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(markup.into_string()))
    } else {
        Ok(HttpResponse::InternalServerError()
            .content_type("text/html; charset=utf-8")
            .body(""))
    }
}

#[get("/recipes/{id}/cancel")]
pub async fn cancel_edit_recipe(
    path: web::Path<i64>,
    client: web::Data<DBClient>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let id = path.into_inner();
    let user = match crate::routes::get_user_or_redirect(&req) {
        Ok(user) => user,
        Err(response) => return Ok(response),
    };
    let client: &DBClient = client.get_ref();

    let recipe = database::recipes::get_recipe(client, id, user.id().to_string()).await;

    if let Ok(recipe) = recipe {
        let markup = view::recipes::recipe_row(&recipe);
        Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(markup.into_string()))
    } else {
        Ok(HttpResponse::InternalServerError()
            .content_type("text/html; charset=utf-8")
            .body(""))
    }
}
