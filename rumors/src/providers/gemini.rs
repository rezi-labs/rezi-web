use super::{AiProvider, GenerationConfig, ProviderError};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct GenerateContentRequest {
    contents: Vec<Content>,
    generation_config: GeminiGenerationConfig,
}

#[derive(Debug, Serialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Debug, Serialize)]
struct Part {
    text: String,
}

#[derive(Debug, Serialize)]
struct GeminiGenerationConfig {
    temperature: f32,
    #[serde(rename = "topP")]
    top_p: f32,
    #[serde(rename = "maxOutputTokens")]
    max_output_tokens: u32,
}

#[derive(Debug, Deserialize)]
struct GenerateContentResponse {
    candidates: Vec<Candidate>,
}

#[derive(Debug, Deserialize)]
struct Candidate {
    content: ResponseContent,
}

#[derive(Debug, Deserialize)]
struct ResponseContent {
    parts: Vec<ResponsePart>,
}

#[derive(Debug, Deserialize)]
struct ResponsePart {
    text: String,
}

#[derive(Debug, Clone)]
pub struct GeminiProvider {
    client: Client,
    api_key: String,
    base_url: String,
}

impl GeminiProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            base_url: "https://generativelanguage.googleapis.com/v1beta".to_string(),
        }
    }

    pub fn with_base_url(mut self, base_url: String) -> Self {
        self.base_url = base_url;
        self
    }
}

#[async_trait]
impl AiProvider for GeminiProvider {
    async fn generate_content(&self, prompt: &str) -> Result<String, ProviderError> {
        let config = GenerationConfig::default();
        self.generate_content_with_config(prompt, &config).await
    }

    async fn generate_content_with_config(
        &self,
        prompt: &str,
        config: &GenerationConfig,
    ) -> Result<String, ProviderError> {
        let url = format!(
            "{}/models/gemini-pro:generateContent?key={}",
            self.base_url, self.api_key
        );

        let request = GenerateContentRequest {
            contents: vec![Content {
                parts: vec![Part {
                    text: prompt.to_string(),
                }],
            }],
            generation_config: GeminiGenerationConfig {
                temperature: config.temperature,
                top_p: config.top_p,
                max_output_tokens: config.max_output_tokens,
            },
        };

        let response = self.client.post(&url).json(&request).send().await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(ProviderError::ApiError {
                message: error_text,
            });
        }

        let response_body: GenerateContentResponse = response.json().await?;

        response_body
            .candidates
            .first()
            .and_then(|candidate| candidate.content.parts.first())
            .map(|part| part.text.clone())
            .ok_or_else(|| ProviderError::ApiError {
                message: "No content in response".to_string(),
            })
    }
}