use actix_web::error::ParseError;
use actix_web::{HttpRequest, HttpResponse, Result, delete, get, patch, post, web};
use log::info;
use maud::{Markup, html};
use serde::Deserialize;
use url::Url;

use crate::config::Server;
use crate::database::recipes::Recipe;
use crate::database::{self, DBClient};
use crate::routes::get_user;
use crate::view::{self, index};
use crate::witch;

#[derive(Deserialize)]
pub struct CreateRecipeRequest {
    pub title: Option<String>,
    pub url: Option<String>,
    pub content: String,
}

#[derive(Deserialize)]
pub struct ProcessRecipeRequest {
    pub url: Option<String>,
    pub content: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateRecipeRequest {
    pub title: Option<String>,
    pub url: Option<String>,
    pub content: Option<String>,
}

#[get("/recipes")]
pub async fn recipe_endpoint(
    server: web::Data<Server>,
    client: web::Data<DBClient>,
    req: HttpRequest,
) -> Result<Markup> {
    let user = get_user(req).unwrap();
    let client = client.get_ref();

    let recipes = database::recipes::get_recipes(client, user.id().to_string())
        .await
        .unwrap_or_default();

    let should_poll_reload = server.db_token().is_none();

    Ok(index(
        Some(view::recipes::recipes(recipes)),
        should_poll_reload,
        Some(&user),
    ))
}

#[post("/recipes")]
pub async fn create_recipe(
    form: web::Form<CreateRecipeRequest>,
    client: web::Data<DBClient>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let client: &DBClient = client.get_ref();
    let user = match crate::routes::get_user_or_redirect(&req) {
        Ok(user) => user,
        Err(response) => return Ok(response),
    };

    let recipe = Recipe {
        id: None,
        owner_id: user.id().to_string(),
        title: form.title.clone(),
        url: form.url.clone(),
        content: form.content.clone(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    let res = database::recipes::create_recipe(client, recipe).await;

    let Ok(recipe) = res else {
        let markup = html! {
            div class="alert alert-error" {
                "Failed to create recipe"
            }
        };
        return Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(markup.into_string()));
    };

    let markup = view::recipes::recipe_row(&recipe);
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(markup.into_string()))
}

#[get("/recipes/{id}")]
pub async fn get_recipe(
    path: web::Path<i64>,
    client: web::Data<DBClient>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let id = path.into_inner();
    let user = match crate::routes::get_user_or_redirect(&req) {
        Ok(user) => user,
        Err(response) => return Ok(response),
    };
    let client: &DBClient = client.get_ref();

    let recipe = database::recipes::get_recipe(client, id, user.id().to_string()).await;

    match recipe {
        Ok(recipe) => {
            let markup = view::recipes::recipe_row(&recipe);
            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(markup.into_string()))
        }
        Err(_) => Ok(HttpResponse::InternalServerError()
            .content_type("text/html; charset=utf-8")
            .body("")),
    }
}

#[patch("/recipes/{id}")]
pub async fn update_recipe(
    path: web::Path<i64>,
    form: web::Form<UpdateRecipeRequest>,
    client: web::Data<DBClient>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let id = path.into_inner();
    let user = match crate::routes::get_user_or_redirect(&req) {
        Ok(user) => user,
        Err(response) => return Ok(response),
    };
    let client: &DBClient = client.get_ref();

    info!("update_recipe: {id}");

    let recipe = database::recipes::update_recipe(
        client,
        id,
        form.title.clone(),
        form.url.clone(),
        form.content.clone(),
        user.id().to_string(),
    )
    .await;

    match recipe {
        Ok(recipe) => {
            let markup = view::recipes::recipe_row(&recipe);
            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(markup.into_string()))
        }
        Err(err) => {
            log::error!("{err}");
            Err(ParseError::Incomplete.into())
        }
    }
}

#[post("/recipes/process")]
pub async fn process_recipe_input(
    form: web::Form<ProcessRecipeRequest>,
    client: web::Data<DBClient>,
    config: web::Data<Server>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let user = match crate::routes::get_user_or_redirect(&req) {
        Ok(user) => user,
        Err(response) => return Ok(response),
    };

    let db_client: &DBClient = client.get_ref();

    // Determine if we're processing a URL or text content
    let (recipe_content, recipe_url) = if let Some(url) = &form.url {
        if !url.trim().is_empty() {
            // Process URL
            let parsed_url = match Url::parse(url) {
                Ok(u) => u,
                Err(_) => {
                    let markup = html! {
                        div class="alert alert-error" {
                            "Invalid URL format. Please enter a valid recipe URL."
                        }
                    };
                    return Ok(HttpResponse::Ok()
                        .content_type("text/html; charset=utf-8")
                        .body(markup.into_string()));
                }
            };

            let content = match witch::hex(parsed_url.to_string()).await {
                Ok(content) => content,
                Err(err) => {
                    log::error!("Failed to fetch URL content: {err}");
                    let markup = html! {
                        div class="alert alert-error" {
                            "Failed to fetch content from URL. Please check the URL and try again."
                        }
                    };
                    return Ok(HttpResponse::Ok()
                        .content_type("text/html; charset=utf-8")
                        .body(markup.into_string()));
                }
            };

            (content, Some(url.clone()))
        } else {
            // Process text content
            match &form.content {
                Some(content) if !content.trim().is_empty() => (content.clone(), None),
                _ => {
                    let markup = html! {
                        div class="alert alert-error" {
                            "Please provide either a recipe URL or recipe text content."
                        }
                    };
                    return Ok(HttpResponse::Ok()
                        .content_type("text/html; charset=utf-8")
                        .body(markup.into_string()));
                }
            }
        }
    } else if let Some(content) = &form.content {
        if !content.trim().is_empty() {
            (content.clone(), None)
        } else {
            let markup = html! {
                div class="alert alert-error" {
                    "Please provide either a recipe URL or recipe text content."
                }
            };
            return Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(markup.into_string()));
        }
    } else {
        let markup = html! {
            div class="alert alert-error" {
                "Please provide either a recipe URL or recipe text content."
            }
        };
        return Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(markup.into_string()));
    };

    // Extract recipe title and generate grocery list
    let use_gemini = config.llm_provider().to_lowercase() == "gemini";
    
    // Try to extract structured recipe data to get the title
    let recipe_title = match crate::llm::extract_recipe_with_llm(
        &recipe_content,
        &config.llm_api_key(),
        use_gemini,
    )
    .await
    {
        Ok(extracted_recipe) => Some(extracted_recipe.title),
        Err(_) => {
            // First fallback: try to extract title from HTML if it's a URL
            if recipe_url.is_some() {
                if let Some(html_title) = crate::scrapy::extract_title(&recipe_content) {
                    Some(html_title)
                } else {
                    // Second fallback: generate title using LLM
                    match crate::llm::generate_title_with_llm(
                        &recipe_content,
                        &config.llm_api_key(),
                        use_gemini,
                    )
                    .await
                    {
                        Ok(generated_title) => Some(generated_title),
                        Err(_) => Some("Untitled Recipe".to_string()),
                    }
                }
            } else {
                // For text content, try to generate a title using LLM
                match crate::llm::generate_title_with_llm(
                    &recipe_content,
                    &config.llm_api_key(),
                    use_gemini,
                )
                .await
                {
                    Ok(generated_title) => Some(generated_title),
                    Err(_) => Some("Untitled Recipe".to_string()),
                }
            }
        }
    };

    // Generate grocery list from recipe content using Rust-based LLM
    let grocery_list = crate::routes::generate_task_response_rust_llm(
        &recipe_content,
        &config.llm_provider(),
        &config.llm_api_key(),
        db_client,
        user.id().to_string(),
    )
    .await;

    // Create and save the recipe
    let recipe = Recipe::new(
        None,
        user.id().to_string(),
        recipe_title,
        recipe_url,
        recipe_content.clone(),
    );

    let recipe_result = database::recipes::create_recipe(db_client, recipe).await;

    let markup = html! {
        div class="space-y-4" {
            div class="alert alert-success" {
                "Recipe processed successfully! Grocery list generated and recipe saved."
            }
            
            div class="card bg-base-100 shadow-lg" {
                div class="card-body" {
                    h3 class="card-title" { "Generated Grocery List" }
                    div class="prose max-w-none" {
                        pre class="whitespace-pre-wrap bg-base-200 p-4 rounded" {
                            (grocery_list)
                        }
                    }
                }
            }

            @if let Ok(saved_recipe) = recipe_result {
                div class="card bg-base-100 shadow-lg" {
                    div class="card-body" {
                        h3 class="card-title" { "Saved Recipe" }
                        p class="text-sm opacity-70" {
                            "Recipe ID: " (saved_recipe.id())
                        }
                        @if let Some(url) = &saved_recipe.url {
                            p class="text-sm" {
                                "Source URL: "
                                a href=(url) target="_blank" class="link" { (url) }
                            }
                        }
                    }
                }
            }
            
            div class="flex gap-2" {
                a href="/recipes" class="btn btn-primary" {
                    "View All Recipes"
                }
                a href="/items" class="btn btn-secondary" {
                    "View Grocery Items"
                }
                button class="btn btn-ghost" 
                       hx-get="/" 
                       hx-target="body" 
                       hx-swap="outerHTML" {
                    "Add Another Recipe"
                }
            }
        }
    };

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(markup.into_string()))
}

#[post("/recipes/extract")]
pub async fn extract_recipe_structure(
    form: web::Form<ProcessRecipeRequest>,
    client: web::Data<DBClient>,
    config: web::Data<Server>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let user = match crate::routes::get_user_or_redirect(&req) {
        Ok(user) => user,
        Err(response) => return Ok(response),
    };

    let db_client: &DBClient = client.get_ref();

    // Determine if we're processing a URL or text content
    let (recipe_content, recipe_url) = if let Some(url) = &form.url {
        if !url.trim().is_empty() {
            // Process URL
            let parsed_url = match Url::parse(url) {
                Ok(u) => u,
                Err(_) => {
                    let markup = html! {
                        div class="alert alert-error" {
                            "Invalid URL format. Please enter a valid recipe URL."
                        }
                    };
                    return Ok(HttpResponse::Ok()
                        .content_type("text/html; charset=utf-8")
                        .body(markup.into_string()));
                }
            };

            let content = match witch::hex(parsed_url.to_string()).await {
                Ok(content) => content,
                Err(err) => {
                    log::error!("Failed to fetch URL content: {err}");
                    let markup = html! {
                        div class="alert alert-error" {
                            "Failed to fetch content from URL. Please check the URL and try again."
                        }
                    };
                    return Ok(HttpResponse::Ok()
                        .content_type("text/html; charset=utf-8")
                        .body(markup.into_string()));
                }
            };

            (content, Some(url.clone()))
        } else {
            // Process text content
            match &form.content {
                Some(content) if !content.trim().is_empty() => (content.clone(), None),
                _ => {
                    let markup = html! {
                        div class="alert alert-error" {
                            "Please provide either a recipe URL or recipe text content."
                        }
                    };
                    return Ok(HttpResponse::Ok()
                        .content_type("text/html; charset=utf-8")
                        .body(markup.into_string()));
                }
            }
        }
    } else if let Some(content) = &form.content {
        if !content.trim().is_empty() {
            (content.clone(), None)
        } else {
            let markup = html! {
                div class="alert alert-error" {
                    "Please provide either a recipe URL or recipe text content."
                }
            };
            return Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(markup.into_string()));
        }
    } else {
        let markup = html! {
            div class="alert alert-error" {
                "Please provide either a recipe URL or recipe text content."
            }
        };
        return Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(markup.into_string()));
    };

    // Extract structured recipe information
    let use_gemini = config.llm_provider().to_lowercase() == "gemini";
    let extracted_recipe = crate::llm::extract_recipe_with_llm(
        &recipe_content,
        &config.llm_api_key(),
        use_gemini,
    )
    .await;

    // Also generate grocery list
    let grocery_list = crate::routes::generate_task_response_rust_llm(
        &recipe_content,
        &config.llm_provider(),
        &config.llm_api_key(),
        db_client,
        user.id().to_string(),
    )
    .await;

    match extracted_recipe {
        Ok(recipe_data) => {
            // Create and save the structured recipe
            let formatted_content = format!(
                "# {}\n\n## Ingredients\n{}\n\n## Instructions\n{}\n\n{}{}{}", 
                recipe_data.title,
                recipe_data.ingredients.iter().map(|i| format!("- {}", i)).collect::<Vec<_>>().join("\n"),
                recipe_data.instructions.iter().enumerate().map(|(i, inst)| format!("{}. {}", i + 1, inst)).collect::<Vec<_>>().join("\n"),
                recipe_data.prep_time.as_ref().map(|pt| format!("**Prep Time:** {}\n", pt)).unwrap_or_default(),
                recipe_data.cook_time.as_ref().map(|ct| format!("**Cook Time:** {}\n", ct)).unwrap_or_default(),
                recipe_data.servings.as_ref().map(|s| format!("**Servings:** {}\n", s)).unwrap_or_default()
            );

            let recipe = Recipe::new(
                None,
                user.id().to_string(),
                Some(recipe_data.title.clone()),
                recipe_url,
                formatted_content,
            );

            let recipe_result = database::recipes::create_recipe(db_client, recipe).await;

            let markup = html! {
                div class="space-y-4" {
                    div class="alert alert-success" {
                        "Recipe extracted and structured successfully! Grocery list generated and recipe saved."
                    }
                    
                    div class="grid grid-cols-1 lg:grid-cols-2 gap-4" {
                        div class="card bg-base-100 shadow-lg" {
                            div class="card-body" {
                                h3 class="card-title" { "Extracted Recipe: " (recipe_data.title) }
                                
                                h4 class="font-semibold mt-4" { "Ingredients" }
                                ul class="list-disc list-inside" {
                                    @for ingredient in &recipe_data.ingredients {
                                        li { (ingredient) }
                                    }
                                }
                                
                                h4 class="font-semibold mt-4" { "Instructions" }
                                ol class="list-decimal list-inside" {
                                    @for instruction in &recipe_data.instructions {
                                        li class="mb-2" { (instruction) }
                                    }
                                }
                                
                                @if let Some(prep_time) = &recipe_data.prep_time {
                                    div class="mt-4" {
                                        span class="font-semibold" { "Prep Time: " }
                                        (prep_time)
                                    }
                                }
                                @if let Some(cook_time) = &recipe_data.cook_time {
                                    div {
                                        span class="font-semibold" { "Cook Time: " }
                                        (cook_time)
                                    }
                                }
                                @if let Some(servings) = &recipe_data.servings {
                                    div {
                                        span class="font-semibold" { "Servings: " }
                                        (servings)
                                    }
                                }
                            }
                        }

                        div class="card bg-base-100 shadow-lg" {
                            div class="card-body" {
                                h3 class="card-title" { "Generated Grocery List" }
                                div class="prose max-w-none" {
                                    pre class="whitespace-pre-wrap bg-base-200 p-4 rounded" {
                                        (grocery_list)
                                    }
                                }
                            }
                        }
                    }

                    @if let Ok(saved_recipe) = recipe_result {
                        div class="alert alert-info" {
                            "Recipe saved with ID: " (saved_recipe.id())
                            @if let Some(url) = &saved_recipe.url {
                                br;
                                "Source URL: "
                                a href=(url) target="_blank" class="link" { (url) }
                            }
                        }
                    }
                    
                    div class="flex gap-2" {
                        a href="/recipes" class="btn btn-primary" {
                            "View All Recipes"
                        }
                        a href="/items" class="btn btn-secondary" {
                            "View Grocery Items"
                        }
                        button class="btn btn-ghost" 
                               hx-get="/" 
                               hx-target="body" 
                               hx-swap="outerHTML" {
                            "Add Another Recipe"
                        }
                    }
                }
            };

            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(markup.into_string()))
        }
        Err(e) => {
            log::error!("Failed to extract recipe: {:?}", e);
            let markup = html! {
                div class="space-y-4" {
                    div class="alert alert-warning" {
                        "Recipe extraction failed, but we still generated a grocery list and saved the raw content."
                    }
                    
                    div class="card bg-base-100 shadow-lg" {
                        div class="card-body" {
                            h3 class="card-title" { "Generated Grocery List" }
                            div class="prose max-w-none" {
                                pre class="whitespace-pre-wrap bg-base-200 p-4 rounded" {
                                    (grocery_list)
                                }
                            }
                        }
                    }

                    div class="alert alert-error" {
                        "Error extracting structured recipe data: " 
                        @match e {
                            crate::llm::LlmError::Request(msg) => (msg),
                            crate::llm::LlmError::Auth(msg) => (msg),
                            crate::llm::LlmError::Parse(msg) => (msg),
                        }
                    }
                    
                    div class="flex gap-2" {
                        a href="/recipes" class="btn btn-primary" {
                            "View All Recipes"
                        }
                        a href="/items" class="btn btn-secondary" {
                            "View Grocery Items"
                        }
                        button class="btn btn-ghost" 
                               hx-get="/" 
                               hx-target="body" 
                               hx-swap="outerHTML" {
                            "Try Again"
                        }
                    }
                }
            };

            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(markup.into_string()))
        }
    }
}

#[delete("/recipes/{id}")]
pub async fn delete_recipe(
    path: web::Path<i64>,
    client: web::Data<DBClient>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let id = path.into_inner();
    let client: &DBClient = client.get_ref();
    let user = match crate::routes::get_user_or_redirect(&req) {
        Ok(user) => user,
        Err(response) => return Ok(response),
    };

    let _ = database::recipes::delete_recipe(client, id, user.id().to_string()).await;
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(""))
}

#[get("/recipes/{id}/edit")]
pub async fn edit_recipe(
    path: web::Path<i64>,
    client: web::Data<DBClient>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let id = path.into_inner();
    let user = match crate::routes::get_user_or_redirect(&req) {
        Ok(user) => user,
        Err(response) => return Ok(response),
    };
    let client: &DBClient = client.get_ref();

    let recipe = database::recipes::get_recipe(client, id, user.id().to_string()).await;

    if let Ok(recipe) = recipe {
        let markup = view::recipes::recipe_edit_row(&recipe);
        Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(markup.into_string()))
    } else {
        Ok(HttpResponse::InternalServerError()
            .content_type("text/html; charset=utf-8")
            .body(""))
    }
}

#[get("/recipes/{id}/cancel")]
pub async fn cancel_edit_recipe(
    path: web::Path<i64>,
    client: web::Data<DBClient>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let id = path.into_inner();
    let user = match crate::routes::get_user_or_redirect(&req) {
        Ok(user) => user,
        Err(response) => return Ok(response),
    };
    let client: &DBClient = client.get_ref();

    let recipe = database::recipes::get_recipe(client, id, user.id().to_string()).await;

    if let Ok(recipe) = recipe {
        let markup = view::recipes::recipe_row(&recipe);
        Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(markup.into_string()))
    } else {
        Ok(HttpResponse::InternalServerError()
            .content_type("text/html; charset=utf-8")
            .body(""))
    }
}
