use rig::client::CompletionClient;
use rig::completion::Prompt;
use rig::providers::{anthropic, gemini, openai};
use serde::{Deserialize, Serialize};

use crate::database::{self, DBClient, items::Item};

#[derive(Debug)]
pub enum LlmError {
    Request(String),
    #[allow(dead_code)]
    Auth(String),
    Parse(String),
}

// Rust-based LLM functionality

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtractedRecipe {
    pub title: String,
    pub ingredients: Vec<String>,
    pub instructions: Vec<String>,
    pub prep_time: Option<String>,
    pub cook_time: Option<String>,
    pub servings: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroceryList {
    pub items: Vec<String>,
}

pub enum LlmProvider {
    OpenAI { api_key: String, model: String },
    Anthropic { api_key: String, model: String },
    Gemini { api_key: String, model: String },
}

pub struct LlmClient {
    provider: LlmProvider,
}

impl LlmClient {
    pub fn new(provider: LlmProvider) -> Self {
        Self { provider }
    }

    pub async fn extract_recipe(&self, content: &str) -> Result<ExtractedRecipe, LlmError> {
        let prompt = format!(
            r#"Extract the recipe information from the following content and return it as JSON.
            
Format the response as a JSON object with these fields:
- title: string (recipe title)
- ingredients: array of strings (each ingredient with quantity)
- instructions: array of strings (step-by-step cooking instructions)
- prep_time: string or null (preparation time)
- cook_time: string or null (cooking time)
- servings: string or null (number of servings)

Content to extract from:
{content}

Return only the JSON object, no additional text."#
        );

        let response_text = self.call_llm_api(&prompt).await?;

        // Try to parse the JSON response
        let recipe: ExtractedRecipe = serde_json::from_str(&response_text).map_err(|e| {
            LlmError::Parse(format!(
                "Failed to parse recipe JSON: {e}\nResponse: {response_text}"
            ))
        })?;

        Ok(recipe)
    }

    pub async fn generate_title(&self, content: &str) -> Result<String, LlmError> {
        let prompt = format!(
            r#"Generate a concise, descriptive title for this recipe or cooking content. The title should be 2-8 words and clearly describe what dish is being made.

Content to generate title for:
{content}

Return only the title text, no quotes or additional formatting."#
        );

        let response_text = self.call_llm_api(&prompt).await?;

        // Clean up the response - remove quotes and trim whitespace
        let title = response_text
            .trim()
            .trim_matches('"')
            .trim_matches('\'')
            .trim()
            .to_string();

        if title.is_empty() {
            return Err(LlmError::Parse("Generated title is empty".to_string()));
        }

        Ok(title)
    }

    pub async fn extract_grocery_list(&self, content: &str) -> Result<Vec<String>, LlmError> {
        let prompt = format!(
            r#"Extract a grocery list from the following recipe or content. Focus only on ingredients that need to be purchased.
            
Return the response as a JSON object with this format:
{{"items": ["ingredient 1", "ingredient 2", ...]}}

Content to extract from:
{content}

Return only the JSON object, no additional text."#
        );

        let response_text = self.call_llm_api(&prompt).await?;

        // Try to parse the JSON response
        let grocery_list: GroceryList = serde_json::from_str(&response_text).map_err(|e| {
            LlmError::Parse(format!(
                "Failed to parse grocery list JSON: {e}\nResponse: {response_text}"
            ))
        })?;

        Ok(grocery_list.items)
    }

    async fn call_llm_api(&self, prompt: &str) -> Result<String, LlmError> {
        let system_message = "You are a helpful assistant that extracts recipe information and grocery lists. Always respond with valid JSON.";

        match &self.provider {
            LlmProvider::OpenAI { api_key, model } => {
                let client = openai::Client::new(api_key);
                let agent = client.agent(model).preamble(system_message).build();

                let response = agent
                    .prompt(prompt)
                    .await
                    .map_err(|e| LlmError::Request(format!("OpenAI API call failed: {e}")))?;

                Ok(response)
            }
            LlmProvider::Anthropic { api_key, model } => {
                let client = anthropic::Client::new(api_key);
                let agent = client.agent(model).preamble(system_message).build();

                let response = agent
                    .prompt(prompt)
                    .await
                    .map_err(|e| LlmError::Request(format!("Anthropic API call failed: {e}")))?;

                Ok(response)
            }
            LlmProvider::Gemini { api_key, model } => {
                let client = gemini::Client::new(api_key);
                let agent = client.agent(model).preamble(system_message).build();

                let response = agent
                    .prompt(prompt)
                    .await
                    .map_err(|e| LlmError::Request(format!("Gemini API call failed: {e}")))?;

                Ok(response)
            }
        }
    }
}

// New functions using the Rust-based LLM client with expanded provider support
pub async fn extract_recipe_with_llm(
    content: &str,
    api_key: &str,
    use_gemini: bool,
) -> Result<ExtractedRecipe, LlmError> {
    let provider = if use_gemini {
        LlmProvider::Gemini {
            api_key: api_key.to_string(),
            model: "gemini-1.5-flash".to_string(),
        }
    } else {
        LlmProvider::OpenAI {
            api_key: api_key.to_string(),
            model: "gpt-3.5-turbo".to_string(),
        }
    };

    let client = LlmClient::new(provider);
    client.extract_recipe(content).await
}

// New function to support multiple LLM providers
#[allow(dead_code)]
pub async fn extract_recipe_with_provider(
    content: &str,
    provider: LlmProvider,
) -> Result<ExtractedRecipe, LlmError> {
    let client = LlmClient::new(provider);
    client.extract_recipe(content).await
}

pub async fn generate_title_with_llm(
    content: &str,
    api_key: &str,
    use_gemini: bool,
) -> Result<String, LlmError> {
    let provider = if use_gemini {
        LlmProvider::Gemini {
            api_key: api_key.to_string(),
            model: "gemini-1.5-flash".to_string(),
        }
    } else {
        LlmProvider::OpenAI {
            api_key: api_key.to_string(),
            model: "gpt-3.5-turbo".to_string(),
        }
    };

    let client = LlmClient::new(provider);
    client.generate_title(content).await
}

// New function to support multiple LLM providers
#[allow(dead_code)]
pub async fn generate_title_with_provider(
    content: &str,
    provider: LlmProvider,
) -> Result<String, LlmError> {
    let client = LlmClient::new(provider);
    client.generate_title(content).await
}

pub async fn extract_grocery_list_with_llm(
    content: &str,
    api_key: &str,
    use_gemini: bool,
    user_id: String,
    db_client: &DBClient,
) -> Result<String, LlmError> {
    let provider = if use_gemini {
        LlmProvider::Gemini {
            api_key: api_key.to_string(),
            model: "gemini-1.5-flash".to_string(),
        }
    } else {
        LlmProvider::OpenAI {
            api_key: api_key.to_string(),
            model: "gpt-3.5-turbo".to_string(),
        }
    };

    let client = LlmClient::new(provider);
    let grocery_items = client.extract_grocery_list(content).await?;

    // Create database items from the grocery list
    let items: Vec<Item> = grocery_items
        .iter()
        .map(|item| Item {
            owner_id: user_id.clone(),
            id: None,
            task: item.clone(),
            completed: 0,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        })
        .collect();

    database::items::create_items(db_client, items).await;

    let items_string = grocery_items.join("\n");
    Ok(format!("Created grocery items:\n{items_string}"))
}

// New function to support multiple LLM providers
#[allow(dead_code)]
pub async fn extract_grocery_list_with_provider(
    content: &str,
    provider: LlmProvider,
    user_id: String,
    db_client: &DBClient,
) -> Result<String, LlmError> {
    let client = LlmClient::new(provider);
    let grocery_items = client.extract_grocery_list(content).await?;

    // Create database items from the grocery list
    let items: Vec<Item> = grocery_items
        .iter()
        .map(|item| Item {
            owner_id: user_id.clone(),
            id: None,
            task: item.clone(),
            completed: 0,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        })
        .collect();

    database::items::create_items(db_client, items).await;

    let items_string = grocery_items.join("\n");
    Ok(format!("Created grocery items:\n{items_string}"))
}

// Helper function to create providers from config strings
#[allow(dead_code)]
pub fn create_llm_provider(
    provider_name: &str,
    api_key: &str,
    model: Option<&str>,
) -> Result<LlmProvider, LlmError> {
    match provider_name.to_lowercase().as_str() {
        "openai" => Ok(LlmProvider::OpenAI {
            api_key: api_key.to_string(),
            model: model.unwrap_or("gpt-3.5-turbo").to_string(),
        }),
        "anthropic" | "claude" => Ok(LlmProvider::Anthropic {
            api_key: api_key.to_string(),
            model: model.unwrap_or("claude-3-haiku-20240307").to_string(),
        }),
        "gemini" | "google" => Ok(LlmProvider::Gemini {
            api_key: api_key.to_string(),
            model: model.unwrap_or("gemini-1.5-flash").to_string(),
        }),
        _ => Err(LlmError::Request(format!(
            "Unsupported LLM provider: {provider_name}"
        ))),
    }
}
