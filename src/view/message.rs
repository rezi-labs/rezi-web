use maud::{Markup, html};

use crate::{database::messages::ChatMessage, unsafe_token_decode::User, view::icons::spark_icon};

pub fn render(message: &ChatMessage, user: Option<User>) -> Markup {
    let initials = match user.clone() {
        Some(u) => u.initials(),
        None => "GY".to_string(),
    };

    let sender = match user {
        Some(s) => s.email().to_string(),
        None => "Grocy".to_string(),
    };

    html! {
        div class={
            @if message.is_user {
                "chat chat-end"
            } @else {
                "chat chat-start"
            }
        } {
            div class="chat-image avatar" {
                div class="w-10 rounded-full" {
                    div class={
                        @if message.is_user {
                            "w-10 h-10 bg-secondary rounded-full flex items-center justify-center text-secondary-content font-bold"
                        } @else {
                            "w-10 h-10 bg-primary rounded-full flex items-center justify-center text-primary-content font-bold"
                        }
                    } {
                        @if message.is_user {
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
                @if message.is_user {
                    "chat-bubble chat-bubble-primary"
                } @else {
                    "chat-bubble"
                }
            } {
                div {
                            (message.content)
                }

            }
            div class="chat-footer opacity-50"{
                  (ai_btn(&message))
            }

        }
    }
}

pub fn ai_btn(message: &ChatMessage) -> Markup {
    let id = format!("ai-btn-{}", message.id());
    let spinner_id = format!("ai-btn-spinner-{}-{}", message.id(), message.is_user());
    html! {
        span {

        form id=(id) {
            input type="hidden" name="message" value=(message.content)  {}

            button class="btn btn-xs btn-accent" type="submit" hx-post="ai/items" hx-target="#chat-messages"  hx-indicator={"#"(spinner_id)} hx-swap="beforeend" hx-on--after-request="this.reset()" {
                (spark_icon())
                "grocery"
            }
        }

        span id=(spinner_id)  class="htmx-indicator loading loading-infinity loading-md" {}
    }}
}
