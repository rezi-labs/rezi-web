use crate::database::{self, DBClient};
use crate::routes::{ChatMessage, get_user};
use crate::{message, user};
use actix_web::{HttpRequest, Result as AwResult};
use actix_web::{get, web};
use maud::{Markup, html};

#[get("")]
pub async fn index_route(client: web::Data<DBClient>, req: HttpRequest) -> AwResult<Markup> {
    let user = get_user(req).unwrap();
    let client = client.get_ref();
    let messages = database::get_messages(client, user.id()).await;

    Ok(super::index(
        Some(render(&messages, &user)),
        messages.as_slice(),
        &user,
    ))
}

pub fn render(messages: &[ChatMessage], user: &user::User) -> Markup {
    html! {
        div class="card bg-base-200 shadow-xl" {
            div class="card-body" {
                h2 class="card-title text-2xl mb-4" {
                    svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24" {
                        path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" {
                        }
                    }
                    "Chat"
                }
                div id="chat-messages" class="chat-container h-96 bg-base-100 p-4 rounded-lg mb-4 space-y-3 overflow-y-auto" {
                    @for message in messages {
                        (message::render(message, Some(user.clone())))
                        (message::render(&message.ai_message(), None))
                    }
                }
                form class="flex gap-2" hx-post="/chat" hx-target="#chat-messages" hx-swap="beforeend" hx-on--after-request="this.reset()" {
                    input class="input input-bordered flex-1" type="text" name="message" placeholder="Type your message..." required;
                    button class="btn btn-primary" type="submit" hx-indicator="#spinner" {
                        svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" {
                            path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8" {
                            }
                        }
                        "Send"
                    }
                    span id="spinner"  class="htmx-indicator loading loading-bars loading-md" {}
                }
            }
        }
    }
}
