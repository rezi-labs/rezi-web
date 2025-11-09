use actix_web::{HttpRequest, Result as AwResult};
use actix_web::{get, web};
use maud::{Markup, html};

pub mod about;
pub mod export;
mod icons;
pub mod items;
pub mod login;
mod navbar;
pub mod profile;
pub mod recipes;

pub use items::render_item;

use crate::config::Server;
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
    let content = content.unwrap_or_else(|| recipe_input_form());
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

fn recipe_input_form() -> Markup {
    html! {
        div class="max-w-2xl mx-auto" {
            div class="card bg-base-200 shadow-xl" {
                div class="card-body" {
                    h2 class="card-title justify-center mb-6" {
                        "Add Recipe"
                    }
                    div class="space-y-6" {
                        div class="form-control" {
                            label class="label" {
                                span class="label-text font-medium" { "Recipe URL" }
                            }
                            form hx-post="/recipes/process" hx-target="#result" hx-swap="innerHTML" {
                                div class="join w-full" {
                                    input 
                                        class="input input-bordered join-item flex-1" 
                                        type="url" 
                                        name="url" 
                                        placeholder="https://example.com/recipe";
                                    button class="btn btn-primary join-item" type="submit" {
                                        "Process URL"
                                    }
                                }
                            }
                            label class="label" {
                                span class="label-text-alt text-base-content/60" { 
                                    "Enter a URL to a recipe page to automatically extract ingredients"
                                }
                            }
                        }
                        
                        div class="divider" { "OR" }
                        
                        div class="form-control" {
                            label class="label" {
                                span class="label-text font-medium" { "Recipe Text" }
                            }
                            form hx-post="/recipes/process" hx-target="#result" hx-swap="innerHTML" {
                                textarea 
                                    class="textarea textarea-bordered min-h-32" 
                                    name="content" 
                                    placeholder="Paste your recipe text here...
                                    
For example:
- 2 cups flour
- 1 cup sugar
- 3 eggs
- 1/2 cup butter";
                                div class="form-control mt-4" {
                                    button class="btn btn-primary w-full" type="submit" {
                                        "Process Recipe Text"
                                    }
                                }
                            }
                            label class="label" {
                                span class="label-text-alt text-base-content/60" { 
                                    "Copy and paste recipe text to extract ingredients and create grocery list"
                                }
                            }
                        }
                    }
                    
                    div id="result" class="mt-6" {
                        // Results will be displayed here
                    }
                }
            }
        }
    }
}
