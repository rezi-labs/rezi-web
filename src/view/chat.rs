use crate::database::WitchResult;
use crate::message::{link_icon, spark_icon};
use crate::routes::ChatMessage;
use crate::view::icons::{chat_icon, random_potion_icon};
use crate::{message, unsafe_token_decode};
use maud::{Markup, html};

pub fn render() -> Markup {
    html! {
        div class="join" {
            button class="btn join-item" hx-get="/grocy" hx-target="#magic" hx-swap="innerHTML" hx-trigger="click, load" {
                (chat_icon())"Grocy"
            }
            button class="btn join-item" hx-get="/witch" hx-target="#magic" hx-swap="innerHTML" {
                (spark_icon())"Witch"
            }
        }
        div id="magic" {

        }
    }
}

pub fn witch_result(result: &WitchResult) -> Markup {
    let fixed_url = if result.url().contains("https://") {
        result.url()
    } else {
        &format!("https://{}", result.url())
    };
    html! {
        li .list-row {
        div {
            (random_potion_icon())
        }
        div {
            div {
                (result.url())
            }
            div class="text-xs font-semibold opacity-60" {
                (result.content())
            }
        }

        a .btn .btn-square .btn-ghost href=(fixed_url)
        target="_blank"
        rel="noopener noreferrer" {
            (link_icon())
        }
        form hx-post="/witch" hx-swap="none" {

            input type="hidden" name="witch_id" value=(result.id()){}
            button class="btn btn-square btn-ghost" type="submit" {
                (spark_icon())
            }
        }

     }
    }
}

pub fn witch(results: &[WitchResult]) -> Markup {
    html! {

        ul id="result-message" class="list h-96 bg-base-200 p-4 rounded-lg mb-4 space-y-3 overflow-y-auto" {
            li class="p-4 pb-2 text-xs opacity-60 tracking-wide" {
                  "The work of the witch"
              }
           @for result in results {
               (witch_result(result))
           }
        }

        form class="flex gap-2" hx-post="/witch" hx-target="#result-message" hx-swap="beforeend" hx-on--after-request="this.reset()" {
            input class="input input-bordered flex-1" type="text" name="url" placeholder="Any url to a recipe..." required;
            button class="btn btn-primary" type="submit" hx-indicator="#spinner" {
                (spark_icon())
                "Hex Hex"
            }
            span id="spinner"  class="htmx-indicator loading loading-bars loading-md" {}
        }
    }
}

pub fn grocy(messages: &[ChatMessage], user: &unsafe_token_decode::User) -> Markup {
    html! {
                div id="chat-messages" class="chat-container h-96 bg-base-200 p-4 rounded-lg mb-4 space-y-3 overflow-y-auto" {
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
