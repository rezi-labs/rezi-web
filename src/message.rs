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
                div {
                            (message.content)
                }

            }
            div class="chat-footer opacity-50"{
                  (ai_btn())
            }

        }
    }
}

pub fn ai_btn() -> Markup {
    html! {    span {
        button class="btn btn-xs btn-accent" hx-post="ai/items" hx-indicator="#spinner"{
            (spark_icon())
            "grocery"
        }
        span id="spinner"  class="htmx-indicator loading loading-bars loading-md" {}
    }}
}

pub fn spark_icon() -> Markup {
    html! {
        svg class="size-3" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" {
            path stroke-linecap="round" stroke-linejoin="round" d="M9.813 15.904 9 18.75l-.813-2.846a4.5 4.5 0 0 0-3.09-3.09L2.25 12l2.846-.813a4.5 4.5 0 0 0 3.09-3.09L9 5.25l.813 2.846a4.5 4.5 0 0 0 3.09 3.09L15.75 12l-2.846.813a4.5 4.5 0 0 0-3.09 3.09ZM18.259 8.715 18 9.75l-.259-1.035a3.375 3.375 0 0 0-2.455-2.456L14.25 6l1.036-.259a3.375 3.375 0 0 0 2.455-2.456L18 2.25l.259 1.035a3.375 3.375 0 0 0 2.456 2.456L21.75 6l-1.035.259a3.375 3.375 0 0 0-2.456 2.456ZM16.894 20.567 16.5 21.75l-.394-1.183a2.25 2.25 0 0 0-1.423-1.423L13.5 18.75l1.183-.394a2.25 2.25 0 0 0 1.423-1.423l.394-1.183.394 1.183a2.25 2.25 0 0 0 1.423 1.423l1.183.394-1.183.394a2.25 2.25 0 0 0-1.423 1.423Z" {
            }
        }
    }
}
