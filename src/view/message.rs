use maud::{Markup, PreEscaped, html};

use crate::{
    database::messages::ChatMessage, from_headers::User, routes::random_id, view::icons::spark_icon,
};

pub fn render(message: &ChatMessage, user: Option<User>) -> Markup {
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
                div class="gap-2" {
                            (rendered_message(message))
                }

            }
            div class="chat-footer opacity-50"{
                  (ai_btn(&message))
            }

        }
    }
}

pub fn rendered_message(message: &ChatMessage) -> Markup {
    let parser = pulldown_cmark::Parser::new(&message.content);

    // Write to a new String buffer.
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
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

            button class="btn btn-xs btn-accent" type="submit" hx-post="ai/items" hx-target="#chat-messages"  hx-indicator={"#"(spinner_id)} hx-swap="beforeend" hx-on--after-request="this.reset()" {
                (spark_icon())
                "grocery"
            }
        }

        span id=(spinner_id)  class="htmx-indicator loading loading-infinity loading-md" {}
    }}
}
