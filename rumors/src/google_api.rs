use reqwest::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GoogleApiError {
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("JSON parsing failed: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("API error: {message}")]
    ApiError { message: String },
    #[error("Missing API key")]
    MissingApiKey,
}

#[derive(Debug, Serialize)]
struct GenerateContentRequest {
    contents: Vec<Content>,
    generation_config: GenerationConfig,
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
struct GenerationConfig {
    temperature: f32,
    top_p: f32,
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
pub struct GoogleApiClient {
    client: Client,
    api_key: String,
    base_url: String,
}

impl GoogleApiClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            base_url: "https://generativelanguage.googleapis.com/v1beta".to_string(),
        }
    }

    pub async fn generate_content(&self, prompt: &str) -> Result<String, GoogleApiError> {
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
            generation_config: GenerationConfig {
                temperature: 0.7,
                top_p: 0.8,
                max_output_tokens: 2048,
            },
        };

        let response = self.client.post(&url).json(&request).send().await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(GoogleApiError::ApiError {
                message: error_text,
            });
        }

        let response_body: GenerateContentResponse = response.json().await?;

        response_body
            .candidates
            .first()
            .and_then(|candidate| candidate.content.parts.first())
            .map(|part| part.text.clone())
            .ok_or_else(|| GoogleApiError::ApiError {
                message: "No content in response".to_string(),
            })
    }
}
