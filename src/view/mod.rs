use actix_web::{HttpRequest, Result as AwResult};
use actix_web::{get, web};
use maud::{Markup, html};

pub mod chat;
mod icons;
mod navbar;
pub mod todolist;

pub use todolist::render_item;

use crate::database::{self, DBClient};
use crate::routes::get_user;
use crate::unsafe_token_decode;
use crate::view::chat::witch;

#[get("/")]
pub async fn index_route(req: HttpRequest) -> AwResult<Markup> {
    let user = get_user(req).unwrap();
    Ok(index(None, &user))
}

#[get("/grocy")]
pub async fn grocy_endpoint(client: web::Data<DBClient>, req: HttpRequest) -> AwResult<Markup> {
    let user = get_user(req).unwrap();
    let client = client.get_ref();
    let messages = database::get_messages(client, user.id()).await;

    Ok(chat::grocy(&messages, &user))
}

#[get("/witch")]
pub async fn witch_endpoint(client: web::Data<DBClient>, req: HttpRequest) -> AwResult<Markup> {
    let user = get_user(req).unwrap();
    let client = client.get_ref();
    let results = database::get_witch_results(client, user.id()).await;

    Ok(witch(results.as_slice()))
}

pub fn css(path: impl Into<String>) -> Markup {
    let path: String = path.into();
    html! {link href=(path) rel="stylesheet" type="text/css";}
}

pub fn js(path: impl Into<String>) -> Markup {
    let path: String = path.into();
    html! {script src=(path) {}}
}

pub fn index(content: Option<Markup>, user: &unsafe_token_decode::User) -> Markup {
    let content = content.unwrap_or_else(chat::render);
    html! {
        (maud::DOCTYPE)
        head {
            meta charset="UTF-8";
            meta name="viewport" content="width=device-width, initial-scale=1.0";
            title {
                "Grocy"
            }
            (js("/assets/tw.js"))
            (js("/assets/theme-switcher.js"))
            (js("/assets/htmx.js"))
            (css("/assets/daisy.css"))
            (css("/assets/themes.css"))
            (css("/assets/app.css"))

        }
        body {
            (js("/assets/htmxListener.js"))
            (navbar::render(user))

            div class="container mx-auto p-2" {
                div class="grid grid-cols-1 gap-6" {
                  (content)
                }
            }
        }
    }
}
