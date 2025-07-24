use crate::database::DBClient2;
use crate::database::recipes::Recipe;
use crate::routes::get_user;
use crate::view::icons::{link_icon, spark_icon};
use actix_web::Result as AwResult;
use actix_web::{HttpRequest, get, web};
use maud::{Markup, html};

pub fn recipes(recipes: Vec<Recipe>) -> Markup {
    html! {
        ul class="list bg-base-100 rounded-box shadow-md" {

            li class="p-4 pb-2 text-xs opacity-60 tracking-wide" {
             div class="join" {
                 button class="btn join-item" { "Recipes" }
                (recipe_search())
             }
            }
            @for recipe in recipes {
                (recipe_row(&recipe))
            }

    }
    }
}

#[get("/recipes")]
pub async fn recipe_endpoint(client: web::Data<DBClient2>, req: HttpRequest) -> AwResult<Markup> {
    let _user = get_user(req).unwrap();
    let _client = client.get_ref();

    // todo: implement recipe retrieval logic

    Ok(super::index(Some(recipes(Recipe::examples()))))
}

pub fn recipe_row(result: &Recipe) -> Markup {
    let fixed_url = if result.url().unwrap_or_default().contains("https://") {
        result.url().unwrap_or_default().to_string()
    } else {
        format!("https://{}", result.url().unwrap_or_default())
    };
    html! {
        li .list-row {
        div {
            ("")
        }
        div {
            div {
                (result.url().unwrap_or_default())
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

fn recipe_search() -> Markup {
    html! {
        label class="input" {
            svg class="h-[1em] opacity-50" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" {
                g stroke-linejoin="round" stroke-linecap="round" stroke-width="2.5" fill="none" stroke="currentColor" {
                    circle cx="11" cy="11" r="8" {
                    }
                    path d="m21 21-4.3-4.3" {
                    }
                }
            }
            input class="grow" type="search" placeholder="Search";
        }
    }
}
