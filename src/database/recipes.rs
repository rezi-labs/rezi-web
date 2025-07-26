use libsql_orm::{Filter, FilterOperator, Model};
use serde::{Deserialize, Serialize};

use crate::database::DBClient;

#[allow(unused)]
#[derive(Model, Debug, Clone, Serialize, Deserialize)]
#[table_name("recipes")]
pub struct Recipe {
    pub id: std::option::Option<i64>,
    pub owner_id: String,
    pub title: Option<String>,
    pub url: Option<String>,
    pub content: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[allow(unused)]
impl Recipe {
    pub fn new(
        id: Option<i64>,
        owner_id: String,
        title: Option<String>,
        url: Option<String>,
        content: String,
    ) -> Self {
        Recipe {
            id,
            owner_id,
            title,
            url,
            content,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }

    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    pub fn url(&self) -> Option<&str> {
        self.url.as_deref()
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn content_as_prompt(&self) -> String {
        format!(
            "Tell the user that they can now use this recipe to generate the items for this recipe {}: {}",
            self.title().unwrap_or("Untitled"),
            self.content()
        )
    }

    pub fn owner_id(&self) -> &str {
        &self.owner_id
    }

    pub fn id(&self) -> i64 {
        self.id.unwrap_or(0)
    }

    pub fn examples() -> Vec<Recipe> {
        vec![
            Recipe::new(
                None,
                "1".to_string(),
                Some("Pumpkin Soup".to_string()),
                None,
                "Boil pumpkin and onions".to_string(),
            ),
            Recipe::new(
                None,
                "2".to_string(),
                Some("Chocolate Cake".to_string()),
                None,
                "Mix flour, sugar, eggs, and chocolate".to_string(),
            ),
        ]
    }

    pub fn update_title(&mut self, title: Option<String>) {
        self.title = title;
        self.updated_at = chrono::Utc::now();
    }

    pub fn update_url(&mut self, url: Option<String>) {
        self.url = url;
        self.updated_at = chrono::Utc::now();
    }

    pub fn update_content(&mut self, content: String) {
        self.content = content;
        self.updated_at = chrono::Utc::now();
    }
}

pub async fn get_recipes(client: &DBClient, owner_id: String) -> Result<Vec<Recipe>, String> {
    log::info!("getting recipes for owner: {owner_id}");

    let db = super::unlock_client(client).await;
    let recipes = Recipe::find_where(
        FilterOperator::Single(Filter::eq("owner_id".to_string(), owner_id.clone())),
        &db,
    )
    .await;
    drop(db);

    match recipes {
        Ok(recipes) => {
            log::info!("found {} recipes for owner: {}", recipes.len(), owner_id);
            Ok(recipes)
        }
        Err(err) => {
            log::error!("Error getting recipes: {err}");
            Err("Could not get recipes".to_string())
        }
    }
}

pub async fn create_recipe(client: &DBClient, recipe: Recipe) -> Result<Recipe, String> {
    let db = super::unlock_client(client).await;

    let res = Recipe::create(&recipe, &db).await;
    drop(db);

    match res {
        Ok(created_recipe) => {
            log::info!("created recipe {}", created_recipe.id());
            Ok(created_recipe)
        }
        Err(err) => {
            log::error!("{err:?}");
            Err("Could not create recipe".to_string())
        }
    }
}

#[allow(unused)]
pub async fn create_recipes(client: &DBClient, recipes: Vec<Recipe>) {
    if recipes.is_empty() {
        return;
    }

    let client = super::unlock_client(client).await;
    let result = Recipe::bulk_create(recipes.as_slice(), &client).await;
    match result {
        Ok(_) => log::info!("created {} recipes", recipes.len()),
        Err(err) => log::error!("could not create recipes: {err}"),
    }
}

pub async fn get_recipe(
    client: &DBClient,
    recipe_id: i64,
    owner_id: String,
) -> Result<Recipe, String> {
    let db = super::unlock_client(client).await;
    let recipe_result = Recipe::find_by_id(recipe_id, &db).await;
    drop(db);

    match recipe_result {
        Ok(Some(recipe)) => {
            if recipe.owner_id() != owner_id {
                return Err("Unauthorized".to_string());
            }
            log::info!("found recipe {}", recipe.id());
            Ok(recipe)
        }
        Ok(None) => {
            log::error!("recipe not found: {recipe_id}");
            Err("Recipe not found".to_string())
        }
        Err(err) => {
            log::error!("database error finding recipe {recipe_id}: {err}");
            Err("Database error".to_string())
        }
    }
}

pub async fn update_recipe(
    client: &DBClient,
    recipe_id: i64,
    title: Option<String>,
    url: Option<String>,
    content: Option<String>,
    owner_id: String,
) -> Result<Recipe, String> {
    let db = super::unlock_client(client).await;
    let recipe_result = Recipe::find_by_id(recipe_id, &db).await;

    let mut recipe = match recipe_result {
        Ok(Some(recipe)) => recipe,
        Ok(None) => {
            drop(db);
            return Err("Recipe not found".to_string());
        }
        Err(err) => {
            drop(db);
            log::error!("Error finding recipe: {err:?}");
            return Err("Database error".to_string());
        }
    };

    if recipe.owner_id() != owner_id {
        drop(db);
        return Err("Unauthorized".to_string());
    }

    // Update fields if provided
    if let Some(new_title) = title {
        recipe.update_title(Some(new_title));
    }
    if let Some(new_url) = url {
        recipe.update_url(Some(new_url));
    }
    if let Some(new_content) = content {
        recipe.update_content(new_content);
    }

    let update_result = recipe.update(&db).await;
    drop(db);

    match update_result {
        Ok(updated_recipe) => {
            log::info!("updated recipe {}", updated_recipe.id());
            Ok(updated_recipe)
        }
        Err(err) => {
            log::error!("could not update recipe: {err}");
            Err("Failed to update recipe".to_string())
        }
    }
}

pub async fn delete_recipe(
    client: &DBClient,
    recipe_id: i64,
    owner_id: String,
) -> Result<(), String> {
    let db = super::unlock_client(client).await;
    let recipe_result = Recipe::find_by_id(recipe_id, &db).await;

    match recipe_result {
        Ok(Some(recipe)) => {
            if recipe.owner_id() != owner_id {
                log::error!("Unauthorized delete attempt for recipe {recipe_id}");
                drop(db);
                return Err("Unauthorized".to_string());
            }

            let delete_result = recipe.delete(&db).await;
            drop(db);

            match delete_result {
                Ok(_) => {
                    log::info!("Successfully deleted recipe {recipe_id}");
                    Ok(())
                }
                Err(err) => {
                    log::error!("Failed to delete recipe {recipe_id}: {err:?}");
                    Err("Failed to delete recipe".to_string())
                }
            }
        }
        Ok(None) => {
            log::error!("Recipe {recipe_id} not found");
            drop(db);
            Err("Recipe not found".to_string())
        }
        Err(err) => {
            log::error!("Error finding recipe {recipe_id}: {err:?}");
            drop(db);
            Err("Database error".to_string())
        }
    }
}
