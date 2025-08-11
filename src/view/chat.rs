use crate::database::messages::ChatMessage;
use crate::from_headers;
use crate::view::icons::{chat_icon, send_icon};
use crate::view::message;
use maud::{Markup, html};

pub fn render() -> Markup {
    html! {
        div class="join" .p-2 {
            button class="btn join-item" hx-get="/chat" hx-target="#magic" hx-swap="innerHTML" hx-trigger="click, load" {
                (chat_icon())"Rezi's Help"
            }
        }
        div id="magic" .p-2 {

        }
    }
}

pub fn chat(messages: &[ChatMessage], user: &from_headers::User) -> Markup {
    html! {
                div id="chat-messages" class="chat-container h-full bg-base-200 p-4 min-h-[200px] max-h-[600px] rounded-lg mb-4 space-y-3 overflow-y-auto" {

                    @for message in messages {
                        @if let Some(_) = message.reply_to_id {
                            // This is a user message replying to something - find the original message
                            @if message.is_user() {
                                // For now, just render without reply context in the initial load
                                // In a full implementation, you'd fetch the reply context here
                                (message::render(message, Some(user.clone())))
                            } @else {
                                (message::render(message, None))
                            }
                        } @else {
                            @if message.is_user() {
                                (message::render(message, Some(user.clone())))
                            } @else {
                                (message::render(message, None))
                            }
                        }
                    }
                }
                form class="flex gap-2" hx-post="/chat" hx-target="#chat-messages" hx-swap="beforeend" hx-on--after-request="this.reset()" {
                    div id="reply-context" class="hidden" {
                        // Reply context will be populated by HTMX
                    }
                    input type="hidden" id="reply-to-id" name="reply_to_id" value="";
                    input class="input input-bordered flex-1" id="reply-input" type="text" name="message" placeholder="Type your message..." required;
                    button class="btn btn-primary" type="submit" hx-indicator="#spinner" {
                        (send_icon())
                    }

                    span id="spinner"  class="htmx-indicator loading loading-infinity loading-md" {}
                }


    }
}
