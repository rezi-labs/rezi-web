use actix_web::error::ParseError;
use actix_web::{HttpRequest, Result, post, web};
use chrono::Utc;
use maud::{Markup, html};
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

#[post("chat")]
pub async fn send_message(
    form: web::Form<SendMessageRequest>,
    client: web::Data<DBClient>,
    config: web::Data<Server>,
    req: HttpRequest,
) -> Result<Markup> {
    let user = super::get_user(req).unwrap();

    log::info!("Received chat message: {}", form.message);
    let db_client: &DBClient = client.get_ref();
    // delay if delay is on
    if config.delay() {
        tokio::time::sleep(std::time::Duration::from_millis(2000)).await;
    }

    let message = form.message.clone();

    // Handle reply context
    let reply_context = if let Some(reply_id_str) = &form.reply_to_id {
        if !reply_id_str.is_empty() {
            if let Ok(reply_id) = reply_id_str.parse::<i64>() {
                database::messages::get_message_by_id(db_client, reply_id).await
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };

    // Build the message content with reply context for the AI
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

    // Return both messages as HTML
    Ok(html! {
            (message::render_with_reply_context(&user_message, Some(user.to_owned()), reply_context.as_ref()))
            (message::render(&user_message.ai_message(), None))
    })
}
