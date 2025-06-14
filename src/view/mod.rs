use actix_web::dev::{Service, WebService};
use actix_web::{get, web,Scope};
use actix_web::{Result as AwResult};
use maud::{html, Markup};


mod todolist;
mod navbar;
mod chat;


pub fn scope()-> Scope{
    web::scope("/ui")
        .service(chat::scope())
        .service(todolist::scope())
        .service(index_route)
}

#[get("")]
async fn index_route() -> AwResult<Markup> {
    Ok(index(None))
}


pub fn css(path: impl Into<String>)-> Markup{
    let path: String = path.into();
    html!{link href=(path) rel="stylesheet" type="text/css";}
}

pub fn js(path: impl Into<String>)-> Markup{
    let path: String = path.into();
    html!{script src=(path) {}}
}

pub fn index(content: Option<Markup>)-> Markup{
    let content = content.unwrap_or(chat::render());
    html! {
        (maud::DOCTYPE)
        head {
            meta charset="UTF-8";
            meta name="viewport" content="width=device-width, initial-scale=1.0";
            title {
                "Todo & Chat App"
            }
            (js("/assets/tw.js"))
            (js("/assets/theme-switcher.js"))
            (js("/assets/htmx.js"))
            (css("/assets/daisy.css"))
            (css("/assets/themes.css"))
            (css("/assets/app.css"))
            
        
        }
        body {
            
            (navbar::render())
            
            div class="container mx-auto p-4" {
                div class="grid grid-cols-1 gap-6" {
                  (content)  
                }
            }
        }
    }
}