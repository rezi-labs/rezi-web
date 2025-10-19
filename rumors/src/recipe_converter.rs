use crate::client::AiClient;
use crate::providers::ProviderError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShoppingItem {
    pub name: String,
    pub quantity: Option<String>,
    pub category: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShoppingList {
    pub items: Vec<ShoppingItem>,
    pub recipe_title: Option<String>,
}

pub struct RecipeConverter {
    client: AiClient,
}

impl RecipeConverter {
    pub fn new(client: AiClient) -> Self {
        Self {
            client,
        }
    }

    pub async fn convert_recipe_to_shopping_list(
        &self,
        recipe_text: &str,
    ) -> Result<ShoppingList, ProviderError> {
        let prompt = format!(
            r#"Convert the following recipe into a shopping list. Return ONLY a JSON object with this exact structure:
{{
  "recipe_title": "Recipe Name",
  "items": [
    {{
      "name": "ingredient name",
      "quantity": "amount and unit",
      "category": "produce/dairy/meat/pantry/etc"
    }}
  ]
}}

Recipe:
{}"#,
            recipe_text
        );

        let response = self.client.generate_content(&prompt).await?;

        let cleaned_response = response
            .trim()
            .strip_prefix("```json")
            .unwrap_or(&response)
            .strip_suffix("```")
            .unwrap_or(&response)
            .trim();

        let shopping_list: ShoppingList =
            serde_json::from_str(cleaned_response).map_err(|e| ProviderError::ApiError {
                message: format!("Failed to parse shopping list JSON: {}", e),
            })?;

        Ok(shopping_list)
    }

    pub async fn extract_ingredients_only(
        &self,
        recipe_text: &str,
    ) -> Result<Vec<String>, ProviderError> {
        let prompt = format!(
            r#"Extract only the ingredient names from this recipe. Return as a simple JSON array of strings:
["ingredient1", "ingredient2", "ingredient3"]

Recipe:
{}"#,
            recipe_text
        );

        let response = self.client.generate_content(&prompt).await?;

        let cleaned_response = response
            .trim()
            .strip_prefix("```json")
            .unwrap_or(&response)
            .strip_suffix("```")
            .unwrap_or(&response)
            .trim();

        let ingredients: Vec<String> =
            serde_json::from_str(cleaned_response).map_err(|e| ProviderError::ApiError {
                message: format!("Failed to parse ingredients JSON: {}", e),
            })?;

        Ok(ingredients)
    }
}
