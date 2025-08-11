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
                        (message::render(message, Some(user.clone())))
                        (message::render(&message.ai_message(), None))
                    }
                }
                form class="flex gap-2" hx-post="/chat" hx-target="#chat-messages" hx-swap="beforeend" hx-on--after-request="this.reset()" {

                    input class="input input-bordered flex-1" type="text" name="message" placeholder="Type your message..." required;
                    button class="btn btn-primary" type="submit" hx-indicator="#spinner" {
                        (send_icon())
                    }

                    span id="spinner"  class="htmx-indicator loading loading-infinity loading-md" {}
                }


    }
}
