use log::info;
use rumors::{ProviderType, RumorsClient};

use crate::database::{self, DBClient, items::Item};

#[derive(Debug)]
pub enum LlmError {
    Provider(String),
}

pub async fn simple_item_response(
    provider_type: ProviderType,
    user_message: &str,
    user_id: String,
    db_client: &DBClient,
) -> Result<String, LlmError> {
    let client = RumorsClient::new(provider_type);

    let recipe_text = format!(
        "Create only grocery items out of this, ignore everything else: {}",
        user_message
    );

    info!("Extracting grocery items from: {}", user_message);

    let ingredients = client
        .extract_ingredients(&recipe_text)
        .await
        .map_err(|e| LlmError::Provider(format!("Failed to extract ingredients: {e}")))?;

    let items: Vec<Item> = ingredients
        .iter()
        .map(|ingredient| Item {
            owner_id: user_id.clone(),
            id: None,
            task: ingredient.clone(),
            completed: 0,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        })
        .collect();

    database::items::create_items(db_client, items).await;

    let items_string = ingredients.join("\n");
    let answer = format!("Created {}", items_string);

    Ok(answer)
}

pub async fn simple_chat_response(
    provider_type: ProviderType,
    user_message: &str,
) -> Result<String, LlmError> {
    let client = RumorsClient::new(provider_type);

    let formatted_message = format!(
        "Only answer in commonmark markdown format. You are Rezi a helpful assistant for recipes, cooking, ingredients and groceries. Here is the message from the user: {}",
        user_message
    );

    info!("Processing chat message: {}", user_message);

    let response = client
        .ask_cooking_question(formatted_message)
        .await
        .map_err(|e| LlmError::Provider(format!("Failed to get chat response: {e}")))?;

    Ok(response)
}
