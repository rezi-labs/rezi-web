use actix_web::{HttpRequest, Result as AwResult};
use actix_web::{get, web};
use maud::{Markup, html};

pub mod chat;
mod icons;
pub mod items;
pub mod login;
pub mod message;
mod navbar;
pub mod profile;
pub mod recipes;

pub use items::render_item;

use crate::config::Server;
use crate::database::{self, DBClient};
use crate::routes::get_user;

#[get("/")]
pub async fn index_route(server: web::Data<Server>) -> AwResult<Markup> {
    let should_poll_reload = server.db_token().is_none();
    Ok(index(None, should_poll_reload))
}

#[get("/chat")]
pub async fn chat_endpoint(client: web::Data<DBClient>, req: HttpRequest) -> AwResult<Markup> {
    let user = get_user(req).unwrap();
    let client = client.get_ref();
    let messages = database::messages::get_messages(client, user.id()).await;
    Ok(chat::chat(&messages, &user))
}

pub fn css(path: impl Into<String>) -> Markup {
    let path: String = path.into();
    html! {link href=(path) rel="stylesheet" type="text/css";}
}

pub fn js(path: impl Into<String>) -> Markup {
    let path: String = path.into();
    html! {script src=(path) {}}
}

pub fn index(content: Option<Markup>, reload_polling_active: bool) -> Markup {
    let content = content.unwrap_or_else(chat::render);
    html! {
        (maud::DOCTYPE)
        head {
            meta charset="UTF-8";
            meta name="viewport" content="width=device-width, initial-scale=1.0";
            title {
                "Rezi"
            }
            (js("/assets/tw.js"))
            (js("/assets/theme-switcher.js"))
            (js("/assets/htmx.js"))
            (css("/assets/daisy.css"))
            (css("/assets/themes.css"))
            (css("/assets/app.css"))
            link rel="icon" href="/assets/grocy.svg" sizes="any" type="image/svg+xml" {}

        }
        body hx-boost="true" {
            (js("/assets/htmxListener.js"))
            (js("/assets/htmx-reload.js"))


            (reload_component(reload_polling_active))


            div class="min-h-screen bg-base-100" {
                (navbar::render())
                main class="container mx-auto px-4 py-6" {
                    (content)
                }
            }
        }
    }
}

pub fn reload_component(reload_polling_active: bool) -> Markup {
    if reload_polling_active {
        html! {
            div hx-get="/reload" hx-trigger="every 1s" hx-swap="none" {}
        }
    } else {
        html! {}
    }
}
