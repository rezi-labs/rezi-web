
use reqwest::Client;
use serde::{Deserialize, Serialize};
use async_openai::{Client as OpenAIClient, config::OpenAIConfig, types::{CreateChatCompletionRequestArgs, ChatCompletionRequestMessage, ChatCompletionRequestSystemMessage, ChatCompletionRequestUserMessage, ChatCompletionRequestSystemMessageContent, ChatCompletionRequestUserMessageContent}};

use crate::database::{self, DBClient, items::Item};

#[derive(Debug)]
pub enum LlmError {
    Request(String),
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
    OpenAI { api_key: String },
    Gemini { api_key: String },
}

pub struct LlmClient {
    provider: LlmProvider,
    client: Client,
}

impl LlmClient {
    pub fn new(provider: LlmProvider) -> Self {
        Self {
            provider,
            client: Client::new(),
        }
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

        let response_text = match &self.provider {
            LlmProvider::OpenAI { api_key } => {
                self.call_openai_compatible_api(api_key, &prompt).await?
            }
            LlmProvider::Gemini { api_key } => {
                self.call_gemini_api(api_key, &prompt).await?
            }
        };

        // Try to parse the JSON response
        let recipe: ExtractedRecipe = serde_json::from_str(&response_text)
            .map_err(|e| LlmError::Parse(format!("Failed to parse recipe JSON: {e}\nResponse: {response_text}")))?;

        Ok(recipe)
    }

    pub async fn generate_title(&self, content: &str) -> Result<String, LlmError> {
        let prompt = format!(
            r#"Generate a concise, descriptive title for this recipe or cooking content. The title should be 2-8 words and clearly describe what dish is being made.

Content to generate title for:
{content}

Return only the title text, no quotes or additional formatting."#
        );

        let response_text = match &self.provider {
            LlmProvider::OpenAI { api_key } => {
                self.call_openai_compatible_api(api_key, &prompt).await?
            }
            LlmProvider::Gemini { api_key } => {
                self.call_gemini_api(api_key, &prompt).await?
            }
        };

        // Clean up the response - remove quotes and trim whitespace
        let title = response_text.trim()
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

        let response_text = match &self.provider {
            LlmProvider::OpenAI { api_key } => {
                self.call_openai_compatible_api(api_key, &prompt).await?
            }
            LlmProvider::Gemini { api_key } => {
                self.call_gemini_api(api_key, &prompt).await?
            }
        };

        // Try to parse the JSON response
        let grocery_list: GroceryList = serde_json::from_str(&response_text)
            .map_err(|e| LlmError::Parse(format!("Failed to parse grocery list JSON: {e}\nResponse: {response_text}")))?;

        Ok(grocery_list.items)
    }

    async fn call_openai_compatible_api(&self, api_key: &str, prompt: &str) -> Result<String, LlmError> {
        let config = OpenAIConfig::new()
            .with_api_key(api_key);

        let client = OpenAIClient::with_config(config);

        let request = CreateChatCompletionRequestArgs::default()
            .model("gpt-3.5-turbo")
            .messages([
                ChatCompletionRequestMessage::System(
                    ChatCompletionRequestSystemMessage {
                        content: ChatCompletionRequestSystemMessageContent::Text(
                            "You are a helpful assistant that extracts recipe information and grocery lists. Always respond with valid JSON.".to_string()
                        ),
                        name: None,
                    }
                ),
                ChatCompletionRequestMessage::User(
                    ChatCompletionRequestUserMessage {
                        content: ChatCompletionRequestUserMessageContent::Text(prompt.to_string()),
                        name: None,
                    }
                ),
            ])
            .build()
            .map_err(|e| LlmError::Request(format!("Failed to build request: {e}")))?;

        let response = client
            .chat()
            .create(request)
            .await
            .map_err(|e| LlmError::Request(format!("OpenAI API call failed: {e}")))?;

        response
            .choices
            .first()
            .and_then(|choice| choice.message.content.as_ref())
            .ok_or_else(|| LlmError::Parse("No content in OpenAI response".to_string()))
            .map(|content| content.clone())
    }

    async fn call_gemini_api(&self, api_key: &str, prompt: &str) -> Result<String, LlmError> {
        #[derive(Serialize)]
        struct GeminiRequest {
            contents: Vec<GeminiContent>,
        }

        #[derive(Serialize)]
        struct GeminiContent {
            parts: Vec<GeminiPart>,
        }

        #[derive(Serialize)]
        struct GeminiPart {
            text: String,
        }

        #[derive(Deserialize)]
        struct GeminiResponse {
            candidates: Vec<GeminiCandidate>,
        }

        #[derive(Deserialize)]
        struct GeminiCandidate {
            content: GeminiResponseContent,
        }

        #[derive(Deserialize)]
        struct GeminiResponseContent {
            parts: Vec<GeminiResponsePart>,
        }

        #[derive(Deserialize)]
        struct GeminiResponsePart {
            text: String,
        }

        let request = GeminiRequest {
            contents: vec![GeminiContent {
                parts: vec![GeminiPart {
                    text: prompt.to_string(),
                }],
            }],
        };

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash-latest:generateContent?key={}",
            api_key
        );

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| LlmError::Request(format!("Failed to send Gemini request: {e}")))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(LlmError::Auth(format!(
                "Gemini API returned status {status}: {error_text}"
            )));
        }

        let gemini_response: GeminiResponse = response
            .json()
            .await
            .map_err(|e| LlmError::Parse(format!("Failed to parse Gemini response: {e}")))?;

        gemini_response
            .candidates
            .first()
            .and_then(|candidate| candidate.content.parts.first())
            .map(|part| part.text.clone())
            .ok_or_else(|| LlmError::Parse("No content in Gemini response".to_string()))
    }
}

// New functions using the Rust-based LLM client
pub async fn extract_recipe_with_llm(
    content: &str,
    api_key: &str,
    use_gemini: bool,
) -> Result<ExtractedRecipe, LlmError> {
    let provider = if use_gemini {
        LlmProvider::Gemini { api_key: api_key.to_string() }
    } else {
        LlmProvider::OpenAI { api_key: api_key.to_string() }
    };

    let client = LlmClient::new(provider);
    client.extract_recipe(content).await
}

pub async fn generate_title_with_llm(
    content: &str,
    api_key: &str,
    use_gemini: bool,
) -> Result<String, LlmError> {
    let provider = if use_gemini {
        LlmProvider::Gemini { api_key: api_key.to_string() }
    } else {
        LlmProvider::OpenAI { api_key: api_key.to_string() }
    };

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
        LlmProvider::Gemini { api_key: api_key.to_string() }
    } else {
        LlmProvider::OpenAI { api_key: api_key.to_string() }
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
