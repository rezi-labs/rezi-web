use crate::database::{self, DBClient, WitchResult};
use crate::message::{link_icon, spark_icon};
use crate::routes::get_user;
use crate::view::icons;
use actix_web::Result as AwResult;
use actix_web::{HttpRequest, Result, get, web};
use maud::{Markup, html};

pub struct Recipe {
    id: String,
    name: String,
    url: Option<String>,
    image_url: Option<String>,
    content: String,
    description: String,
    owner_id: String,
}

impl Recipe {
    pub fn new(
        id: String,
        name: String,
        url: Option<String>,
        image_url: Option<String>,
        content: String,
        description: String,
        owner_id: String,
    ) -> Self {
        Recipe {
            id,
            name,
            url,
            image_url,
            content,
            description,
            owner_id,
        }
    }

    pub fn url(&self) -> &str {
        self.url.as_deref().unwrap_or("https://example.com")
    }

    pub fn image_url(&self) -> &str {
        self.image_url
            .as_deref()
            .unwrap_or("https://example.com/image.jpg")
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn owner_id(&self) -> &str {
        &self.owner_id
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn examples() -> Vec<Recipe> {
        vec![
            Recipe::new(
                "".to_string(),
                "Pumpkin Soup".to_string(),
                None,
                None,
                "Boil pumpkin and onions".to_string(),
                "A delicious soup made from pumpkin and onions".to_string(),
                "1".to_string(),
            ),
            Recipe::new(
                "".to_string(),
                "Chocolate Cake".to_string(),
                None,
                None,
                "Mix flour, sugar, eggs, and chocolate".to_string(),
                "A rich and decadent chocolate cake".to_string(),
                "2".to_string(),
            ),
        ]
    }
}

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
pub async fn recipe_endpoint(client: web::Data<DBClient>, req: HttpRequest) -> AwResult<Markup> {
    let _user = get_user(req).unwrap();
    let _client = client.get_ref();

    // todo: implement recipe retrieval logic

    Ok(super::index(Some(recipes(Recipe::examples()))))
}

pub fn recipe_row(result: &Recipe) -> Markup {
    let fixed_url = if result.url().contains("https://") {
        result.url()
    } else {
        &format!("https://{}", result.url())
    };
    html! {
        li .list-row {
        div {
            ("")
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
