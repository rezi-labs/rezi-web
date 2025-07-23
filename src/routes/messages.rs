use actix_web::{HttpRequest, Result, post, web};
use chrono::Utc;
use maud::{Markup, html};
use serde::Deserialize;
use url::Url;

use crate::config::Server;
use crate::database::{self, DBClient};
use crate::view::message;
use crate::witch;

#[derive(Deserialize)]
pub struct SendMessageRequest {
    pub message: String,
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

            super::generate_task_response(
                &hex,
                &config.nest_api(),
                &config.nest_api_key(),
                db_client,
                user.id().to_string(),
            )
            .await
        }
        Err(_) => {
            super::generate_ai_response(&form.message, &config.nest_api(), &config.nest_api_key())
                .await
        }
    };

    let chat_id = super::random_id();

    let user_message = database::messages::ChatMessage {
        id: chat_id,
        content: form.message.clone(),
        ai_response: ai_response.clone(),
        sender: user.id().to_string(),
        timestamp: Utc::now(),
        is_user: true,
    };
    database::messages::save_message(db_client, user_message.clone()).await;

    // do not save ai message
    let ai_message = database::messages::ChatMessage {
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
