pub mod chat;
pub mod client;
pub mod google_api;
pub mod providers;
pub mod recipe_converter;

pub use chat::{ChatSession, RecipeChat};
pub use client::AiClient;
pub use google_api::{GoogleApiClient, GoogleApiError};
pub use providers::{create_provider, ProviderError, ProviderType};
pub use recipe_converter::{RecipeConverter, ShoppingItem, ShoppingList};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RumorsResponse {
    ShoppingList(ShoppingList),
    TextResponse(String),
}

#[derive(Debug, Clone)]
enum FunctionIntent {
    AskCookingQuestion,
    DiscussRecipe,
    AnalyzeRecipe,
    SuggestModifications,
}

pub struct RumorsClient {
    recipe_converter: RecipeConverter,
    recipe_chat: RecipeChat,
}

impl RumorsClient {
    pub fn new(provider_type: ProviderType) -> Self {
        let provider = create_provider(provider_type);
        let client = AiClient::new(provider);
        
        Self {
            recipe_converter: RecipeConverter::new(client.clone()),
            recipe_chat: RecipeChat::new(client),
        }
    }

    // Legacy constructor for backwards compatibility
    pub fn new_with_google_api(api_key: String) -> Self {
        Self::new(ProviderType::GoogleGemini { api_key })
    }

    // New constructor for Ollama
    pub fn new_with_ollama(base_url: String, model: String) -> Self {
        Self::new(ProviderType::Ollama { base_url, model })
    }

    pub async fn convert_recipe_to_shopping_list(
        &self,
        recipe_text: &str,
    ) -> Result<ShoppingList, ProviderError> {
        self.recipe_converter
            .convert_recipe_to_shopping_list(recipe_text)
            .await
    }

    pub async fn extract_ingredients(
        &self,
        recipe_text: &str,
    ) -> Result<Vec<String>, ProviderError> {
        self.recipe_converter
            .extract_ingredients_only(recipe_text)
            .await
    }

    pub async fn ask_cooking_question(
        &self,
        question: String,
    ) -> Result<String, ProviderError> {
        self.recipe_chat.ask(question).await
    }

    pub async fn discuss_recipe(
        &self,
        recipe_text: &str,
        question: String,
    ) -> Result<String, ProviderError> {
        self.recipe_chat.discuss_recipe(recipe_text, question).await
    }

    pub async fn analyze_recipe(&self, recipe_text: &str) -> Result<String, ProviderError> {
        self.recipe_chat.analyze_recipe(recipe_text).await
    }

    pub async fn suggest_recipe_modifications(
        &self,
        recipe_text: &str,
        dietary_requirements: &str,
    ) -> Result<String, ProviderError> {
        self.recipe_chat
            .suggest_recipe_modifications(recipe_text, dietary_requirements)
            .await
    }

    // Main function 1: Dedicated shopping list extraction
    pub async fn extract_shopping_list(&self, recipe_text: &str) -> Result<ShoppingList, ProviderError> {
        self.convert_recipe_to_shopping_list(recipe_text).await
    }

    // Main function 2: Intelligent router that determines which function to use
    pub async fn process_request(&self, input: &str, recipe_text: Option<&str>) -> Result<RumorsResponse, ProviderError> {
        let intent = self.classify_intent(input, recipe_text.is_some()).await?;
        
        match intent {
            FunctionIntent::AskCookingQuestion => {
                let response = self.ask_cooking_question(input.to_string()).await?;
                Ok(RumorsResponse::TextResponse(response))
            },
            FunctionIntent::DiscussRecipe => {
                if let Some(recipe) = recipe_text {
                    let response = self.discuss_recipe(recipe, input.to_string()).await?;
                    Ok(RumorsResponse::TextResponse(response))
                } else {
                    let response = self.ask_cooking_question(input.to_string()).await?;
                    Ok(RumorsResponse::TextResponse(response))
                }
            },
            FunctionIntent::AnalyzeRecipe => {
                if let Some(recipe) = recipe_text {
                    let response = self.analyze_recipe(recipe).await?;
                    Ok(RumorsResponse::TextResponse(response))
                } else {
                    let response = self.ask_cooking_question(
                        "I need a recipe to analyze. Please provide one or ask a general cooking question.".to_string()
                    ).await?;
                    Ok(RumorsResponse::TextResponse(response))
                }
            },
            FunctionIntent::SuggestModifications => {
                if let Some(recipe) = recipe_text {
                    // Extract dietary requirements from the input
                    let dietary_requirements = self.extract_dietary_requirements(input).await?;
                    let response = self.suggest_recipe_modifications(recipe, &dietary_requirements).await?;
                    Ok(RumorsResponse::TextResponse(response))
                } else {
                    let response = self.ask_cooking_question(
                        "I need a recipe to modify. Please provide one or ask a general cooking question.".to_string()
                    ).await?;
                    Ok(RumorsResponse::TextResponse(response))
                }
            },
        }
    }

    // Helper function to classify user intent
    async fn classify_intent(&self, input: &str, has_recipe: bool) -> Result<FunctionIntent, ProviderError> {
        let classification_prompt = format!(
            r#"Classify the following user input into one of these categories. Return ONLY the category name:

Categories:
1. "ask_cooking_question" - General cooking questions, techniques, ingredients, or tips
2. "discuss_recipe" - Questions about a specific recipe, ingredients, or cooking steps
3. "analyze_recipe" - Requests to analyze, review, or provide insights about a recipe
4. "suggest_modifications" - Requests to modify a recipe for dietary needs, preferences, or substitutions

User input: "{input}"
Has recipe provided: {has_recipe}

Category:"#
        );

        let response = self.recipe_chat.ask(classification_prompt).await?;
        let intent = response.trim().to_lowercase();

        match intent.as_str() {
            "discuss_recipe" => Ok(FunctionIntent::DiscussRecipe),
            "analyze_recipe" => Ok(FunctionIntent::AnalyzeRecipe),
            "suggest_modifications" => Ok(FunctionIntent::SuggestModifications),
            _ => Ok(FunctionIntent::AskCookingQuestion), // Default fallback
        }
    }

    // Helper function to extract dietary requirements from input  
    async fn extract_dietary_requirements(&self, input: &str) -> Result<String, ProviderError> {
        let extraction_prompt = format!(
            r#"Extract the dietary requirements, preferences, or modifications requested from this input. 
If no specific dietary requirements are mentioned, return "general modifications".

Input: "{input}"

Dietary requirements:"#
        );

        let response = self.recipe_chat.ask(extraction_prompt).await?;
        Ok(response.trim().to_string())
    }
}
