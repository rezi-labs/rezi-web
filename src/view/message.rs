use maud::{Markup, PreEscaped, html};
use markdown;

use crate::{
    database::messages::ChatMessage,
    routes::random_id,
    user::User,
    view::icons::{reply_icon, spark_icon},
};

pub fn render(message: &ChatMessage, user: Option<User>) -> Markup {
    render_with_reply_context(message, user, None)
}

pub fn render_with_reply_context(
    message: &ChatMessage,
    user: Option<User>,
    reply_to: Option<&ChatMessage>,
) -> Markup {
    if message.content.is_empty() {
        return html!();
    }
    let initials = match user.clone() {
        Some(u) => u.initials(),
        None => "GY".to_string(),
    };

    let sender = match user {
        Some(s) => s.email().to_string(),
        None => "Rezi".to_string(),
    };

    html! {
        div class={
            @if message.is_user() {
                "chat chat-end"
            } @else {
                "chat chat-start"
            }
        } {
            div class="chat-image avatar" {
                div class="w-10 rounded-full" {
                    div class={
                        @if message.is_user() {
                            "w-10 h-10 bg-secondary rounded-full flex items-center justify-center text-secondary-content font-bold"
                        } @else {
                            "w-10 h-10 bg-primary rounded-full flex items-center justify-center text-primary-content font-bold"
                        }
                    } {
                        @if message.is_user() {
                            (initials)
                        } @else {
                            img class="scale-200" src="assets/grocy_close.svg" alt="Grocy Logo"{}
                        }
                    }
                }
            }
            div class="chat-header" {
                (sender)
                time class="text-xs opacity-50" {
                    (message.created_at.format("%H:%M"))
                }
            }
            div class={
                @if message.is_user() {
                    "chat-bubble chat-bubble-primary"
                } @else {
                    "chat-bubble"
                }
            } {
                @if let Some(reply_msg) = reply_to {
                    div class="reply-context bg-base-300 p-2 rounded mb-2 border-l-4 border-primary text-sm" {
                        div class="font-semibold text-base-content" {
                            @if reply_msg.is_user() {
                                "You"
                            } @else {
                                "Rezi"
                            }
                        }
                        div class="text-base-content opacity-80" {
                            (truncated_content(&reply_msg.content, 100))
                        }
                    }
                }
                div class="gap-2" {
                            (rendered_message(message))
                }

            }
            div class="chat-footer opacity-50 flex gap-2"{
                  (ai_btn(&message))
                  (reply_btn(&message))
            }

        }
    }
}

pub fn rendered_message(message: &ChatMessage) -> Markup {
    let html_output = markdown::to_html(&message.content);
    html! {
     (PreEscaped(html_output))
    }
}

pub fn ai_btn(message: &ChatMessage) -> Markup {
    let id = format!("ai-btn-{}", message.id());
    let spinner_id = format!(
        "ai-btn-spinner-{}-{}-{}",
        message.id(),
        message.is_user(),
        random_id()
    );
    html! {
        span {

        form id=(id) {
            input type="hidden" name="message" value=(message.content)  {}

            button class="btn btn-sm btn-primary grocery-btn" type="submit" hx-post="ai/items" hx-target="#chat-messages"  hx-indicator={"#"(spinner_id)} hx-swap="beforeend" hx-on--after-request="this.reset()" {
                span class="grocery-btn-icon" {
                    (spark_icon())
                }
                span class="grocery-btn-text" {
                    "Add to Grocery"
                }
            }
        }

        span id=(spinner_id)  class="htmx-indicator loading loading-infinity loading-md" {}
    }}
}

pub fn truncated_content(content: &str, max_length: usize) -> String {
    if content.len() <= max_length {
        content.to_string()
    } else {
        format!("{}...", &content[..max_length])
    }
}

pub fn reply_btn(message: &ChatMessage) -> Markup {
    let message_id = message.id();

    html! {
        button class="btn btn-sm btn-secondary reply-btn"
               hx-post="/chat/reply"
               hx-target="#reply-context"
               hx-swap="innerHTML"
               hx-vals=(format!(r#"{{"message_id": "{}", "content": {}}}"#, message_id, serde_json::to_string(&message.content).unwrap_or_default())) {
            span class="reply-btn-icon" {
                (reply_icon())
            }
            span class="reply-btn-text" {
                "Reply"
            }
        }
    }
}
