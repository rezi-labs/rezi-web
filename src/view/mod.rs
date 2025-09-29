use actix_web::{HttpRequest, Result as AwResult};
use actix_web::{get, web};
use maud::{Markup, html};

pub mod about;
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
pub async fn index_route(server: web::Data<Server>, req: HttpRequest) -> AwResult<Markup> {
    let should_poll_reload = server.db_token().is_none();
    let user = get_user(req);
    match user {
        Some(ref user) => Ok(index(None, should_poll_reload, Some(user))),
        None => Ok(index(Some(about::about()), should_poll_reload, None)),
    }
}

#[get("/chat")]
pub async fn chat_endpoint(client: web::Data<DBClient>, req: HttpRequest) -> AwResult<Markup> {
    let user = get_user(req).expect("no user");
    let client = client.get_ref();
    let messages = database::messages::get_messages(client, user.id()).await;
    Ok(chat::chat(&messages, &user))
}

#[get("/about/readme")]
pub async fn about_readme_endpoint() -> AwResult<Markup> {
    Ok(about::readme())
}

#[get("/about/changelog")]
pub async fn about_changelog_endpoint() -> AwResult<Markup> {
    Ok(about::changelog())
}

#[get("/about")]
pub async fn about_endpoint(req: HttpRequest) -> AwResult<Markup> {
    let user = get_user(req);
    Ok(index(Some(about::about()), false, user.as_ref()))
}

pub fn css(path: impl Into<String>) -> Markup {
    let path: String = path.into();
    html! {link href=(path) rel="stylesheet" type="text/css";}
}

pub fn js(path: impl Into<String>) -> Markup {
    let path: String = path.into();
    html! {script src=(path) {}}
}

pub fn index(
    content: Option<Markup>,
    _reload_polling_active: bool,
    user: Option<&crate::user::User>,
) -> Markup {
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


            div class="min-h-screen bg-base-100" {
                (navbar::render(user))
                main class="container mx-auto px-4 py-6" {
                    (content)
                }
            }
        }
    }
}
