use maud::{Markup, html};

use crate::{routes::ChatMessage, unsafe_token_decode::User};

pub fn render(message: &ChatMessage, user: Option<User>) -> Markup {
    let initials = match user.clone() {
        Some(u) => u.initials(),
        None => "GY".to_string(),
    };

    let sender = match user {
        Some(s) => s.email().to_string(),
        None => "Agent".to_string(),
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
                        (initials)
                    }
                }
            }
            div class="chat-header" {
                (sender)
                time class="text-xs opacity-50" {
                    (message.timestamp.format("%H:%M"))
                }
            }
            div class={
                @if message.is_user {
                    "chat-bubble chat-bubble-primary"
                } @else {
                    "chat-bubble"
                }
            } {
                (message.content)
            }
        }
    }
}
