pub mod gemini;
pub mod ollama;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProviderError {
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("JSON parsing failed: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("API error: {message}")]
    ApiError { message: String },
    #[error("Missing API key")]
    MissingApiKey,
    #[error("Configuration error: {message}")]
    ConfigError { message: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationConfig {
    pub temperature: f32,
    pub top_p: f32,
    pub max_output_tokens: u32,
}

impl Default for GenerationConfig {
    fn default() -> Self {
        Self {
            temperature: 0.7,
            top_p: 0.8,
            max_output_tokens: 2048,
        }
    }
}

#[async_trait]
pub trait AiProvider: Send + Sync {
    async fn generate_content(&self, prompt: &str) -> Result<String, ProviderError>;
    async fn generate_content_with_config(
        &self,
        prompt: &str,
        config: &GenerationConfig,
    ) -> Result<String, ProviderError>;
}

#[derive(Debug, Clone)]
pub enum ProviderType {
    GoogleGemini { api_key: String },
    Ollama { base_url: String, model: String },
}

pub fn create_provider(provider_type: ProviderType) -> Box<dyn AiProvider> {
    match provider_type {
        ProviderType::GoogleGemini { api_key } => {
            Box::new(gemini::GeminiProvider::new(api_key))
        }
        ProviderType::Ollama { base_url, model } => {
            Box::new(ollama::OllamaProvider::new(base_url, model))
        }
    }
}