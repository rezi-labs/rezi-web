use actix_web::web::Data;
use actix_web::{HttpMessage, HttpRequest, HttpResponse};
use log::error;
use rand::Rng;

use crate::database::DBClient;
use crate::{llm, user};

pub mod assets;
pub mod auth;
pub mod export;
pub mod items;
pub mod messages;
pub mod recipes;
pub mod technical;

pub fn get_user(req: HttpRequest) -> Option<user::User> {
    req.extensions()
        .get::<Data<user::User>>()
        .map(|u| u.as_ref().clone())
}

pub fn get_user_or_redirect(req: &HttpRequest) -> Result<user::User, HttpResponse> {
    match get_user(req.clone()) {
        Some(user) => Ok(user),
        None => {
            let is_htmx = req.headers().contains_key("hx-request");
            if is_htmx {
                Err(HttpResponse::Unauthorized()
                    .content_type("text/html")
                    .body(r#"<div class="alert alert-error">Session expired. Please <a href="/auth/login" class="link">login</a> again.</div>"#))
            } else {
                Err(HttpResponse::Found()
                    .append_header(("Location", "/auth/login"))
                    .finish())
            }
        }
    }
}

pub fn random_html_safe_id() -> u64 {
    let mut rng = rand::rng();
    rng.random::<u64>()
}

pub fn random_id() -> i64 {
    let mut rng = rand::rng();
    rng.random::<i64>()
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

            r"# Error

Something went wrong contacting the agent

                "
            .to_string()
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
