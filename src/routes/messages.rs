use actix_web::error::ParseError;
use actix_web::{HttpRequest, HttpResponse, Result, post, web};
use chrono::Utc;
use maud::html;
use serde::Deserialize;
use url::Url;

use crate::config::Server;
use crate::database::recipes::Recipe;
use crate::database::{self, DBClient};
use crate::view::message;
use crate::witch;

#[derive(Deserialize)]
pub struct SendMessageRequest {
    pub message: String,
    pub is_content: Option<bool>,
    pub is_url: Option<bool>,
    pub reply_to_id: Option<String>,
}

#[derive(Deserialize)]
pub struct ReplyRequest {
    pub message_id: String,
    pub content: String,
}

#[post("chat")]
pub async fn send_message(
    form: web::Form<SendMessageRequest>,
    client: web::Data<DBClient>,
    config: web::Data<Server>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let user = match super::get_user_or_redirect(&req) {
        Ok(user) => user,
        Err(response) => return Ok(response),
    };

    log::info!("Received chat message: {}", form.message);
    let db_client: &DBClient = client.get_ref();
    if config.delay() {
        tokio::time::sleep(std::time::Duration::from_millis(2000)).await;
    }

    let message = form.message.clone();

    let reply_context = if let Some(reply_id_str) = &form.reply_to_id {
        if !reply_id_str.is_empty() {
            if let Ok(reply_id) = reply_id_str.parse::<i64>() {
                database::messages::get_message_by_id(db_client, reply_id, user.id()).await
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };

    let ai_message_content = if let Some(ref reply_msg) = reply_context {
        format!(
            "REPLY TO: [{}]: \"{}\"\n\nUSER MESSAGE: {}",
            if reply_msg.is_user() {
                "User"
            } else {
                "Assistant"
            },
            reply_msg.content,
            message
        )
    } else {
        message.clone()
    };

    let url = Url::parse(&ai_message_content);
    let ai_response = match url {
        Ok(url) => {
            let url = url.as_str().to_string();

            let hex = witch::hex(url.clone()).await;

            let Ok(hex) = hex else {
                return Err(actix_web::error::ErrorBadRequest(
                    "Could not fetch hex".to_string(),
                ));
            };

            let tasks = super::generate_task_response(
                &hex,
                &config.nest_api(),
                &config.nest_api_key(),
                db_client,
                user.id().to_string(),
            )
            .await;

            if !form.is_url.unwrap_or_default() {
                let recipe = Recipe::new(
                    None,
                    user.id().to_string(),
                    Some("Generated".to_string()),
                    Some(url),
                    tasks.clone(),
                );

                let res = database::recipes::create_recipe(db_client, recipe).await;

                if res.is_err() {
                    return Err(actix_web::error::ErrorBadRequest(
                        "Could not create recipe".to_string(),
                    ));
                }
            }

            tasks
        }
        Err(_) => {
            if form.is_content.unwrap_or_default() {
                // If content of a recipe is being sent, extract items from the content
                super::generate_task_response(
                    &ai_message_content,
                    &config.nest_api(),
                    &config.nest_api_key(),
                    db_client,
                    user.id().to_string(),
                )
                .await
            } else {
                super::generate_ai_response(
                    &ai_message_content,
                    &config.nest_api(),
                    &config.nest_api_key(),
                )
                .await
            }
        }
    };

    let checked_user_response = if form.is_url.unwrap_or_default() {
        "".to_string()
    } else {
        form.message.clone()
    };

    let reply_to_id = if let Some(reply_id_str) = &form.reply_to_id {
        if !reply_id_str.is_empty() {
            reply_id_str.parse::<i64>().ok()
        } else {
            None
        }
    } else {
        None
    };

    let user_message = database::messages::ChatMessage {
        id: None,
        content: checked_user_response.clone(),
        ai_response: ai_response.clone(),
        owner_id: user.id().to_string(),
        created_at: Utc::now(),
        is_user: 1,
        reply_to_id,
    };
    let Ok(user_message) = database::messages::save_message(db_client, user_message.clone()).await
    else {
        return Err(ParseError::Incomplete.into());
    };

    // Save the AI response as a separate message
    let ai_message = database::messages::ChatMessage {
        id: None,
        content: ai_response.clone(),
        ai_response: ai_response.clone(),
        owner_id: user.id().to_string(),
        created_at: Utc::now(),
        is_user: 0,
        reply_to_id: None,
    };
    let Ok(ai_message) = database::messages::save_message(db_client, ai_message).await else {
        return Err(ParseError::Incomplete.into());
    };

    // Return both messages as HTML
    let markup = html! {
            (message::render_with_reply_context(&user_message, Some(user.to_owned()), reply_context.as_ref()))
            (message::render(&ai_message, None))
            script {
                "document.getElementById('reply-context').innerHTML = '<div class=\"hidden\"></div>';"
                "document.getElementById('reply-to-id').value = '';"
            }
    };

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(markup.into_string()))
}

#[post("chat/reply")]
pub async fn set_reply(
    form: web::Form<ReplyRequest>,
    client: web::Data<DBClient>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let user = match super::get_user_or_redirect(&req) {
        Ok(user) => user,
        Err(response) => return Ok(response),
    };
    let db_client: &DBClient = client.get_ref();

    let message_id = match form.message_id.parse::<i64>() {
        Ok(id) => id,
        Err(_) => return Err(actix_web::error::ErrorBadRequest("Invalid message ID")),
    };

    let message = database::messages::get_message_by_id(db_client, message_id, user.id()).await;

    match message {
        Some(_msg) => {
            let truncated_content = if form.content.len() > 50 {
                format!("{}...", &form.content[..50])
            } else {
                form.content.clone()
            };

            let markup = html! {
                div class="bg-base-300 p-2 rounded mb-2 border-l-4 border-primary text-sm" {
                    div class="flex justify-between items-center" {
                        div {
                            span class="font-semibold text-base-content" { "Replying to: " }
                            span class="text-base-content opacity-80" { (truncated_content) }
                        }
                        button type="button" class="btn btn-xs btn-ghost"
                               hx-post="/chat/clear-reply"
                               hx-target="#reply-context"
                               hx-swap="innerHTML" { "×" }
                    }
                }
                script {
                    "document.getElementById('reply-to-id').value = '" (message_id) "';"
                    "document.getElementById('reply-input').focus();"
                }
            };

            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(markup.into_string()))
        }
        None => Err(actix_web::error::ErrorNotFound("Message not found")),
    }
}

#[post("chat/clear-reply")]
pub async fn clear_reply(req: HttpRequest) -> Result<HttpResponse> {
    let _user = match super::get_user_or_redirect(&req) {
        Ok(user) => user,
        Err(response) => return Ok(response),
    };

    let markup = html! {
        div class="hidden" {
            // Empty reply context
        }
        script {
            "document.getElementById('reply-to-id').value = '';"
        }
    };

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(markup.into_string()))
}
