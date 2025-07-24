use crate::database::DBClient;
use crate::view::index;
use actix_web::Result as AwResult;
use actix_web::{HttpRequest, get, web};
use maud::{Markup, html};

#[get("/info")]
pub async fn info_endpoint(_client: web::Data<DBClient>, _req: HttpRequest) -> AwResult<Markup> {
    Ok(index(Some(info())))
}

pub fn info() -> Markup {
    html! {
        div class="mx-4" {
            (info_card(
                "Chat",
                "Use the Grocy chat to talk about recipes",
                "Chat",
                ""
            ))
        }
    }
}

pub fn info_card(title: &str, description: &str, link_title: &str, link: &str) -> Markup {
    html! {
        div class="card w-96 bg-base-100 card-xs shadow-sm" {
            div class="card-body" {
                h2 class="card-title" {
                    (title)
                }
                p {
                    (description)
                }
                div class="justify-end card-actions" {
                    a class="btn btn-primary" href=(format!("/{}", link)) {
                        (link_title)
                    }
                }
            }
        }
    }
}
