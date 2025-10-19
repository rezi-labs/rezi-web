use super::{AiProvider, GenerationConfig, ProviderError};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct OllamaGenerateRequest {
    model: String,
    prompt: String,
    stream: bool,
    options: OllamaOptions,
}

#[derive(Debug, Serialize)]
struct OllamaOptions {
    temperature: f32,
    top_p: f32,
    num_predict: i32,
}

#[derive(Debug, Deserialize)]
struct OllamaGenerateResponse {
    response: String,
    done: bool,
}

#[derive(Debug, Clone)]
pub struct OllamaProvider {
    client: Client,
    base_url: String,
    model: String,
}

impl OllamaProvider {
    pub fn new(base_url: String, model: String) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.trim_end_matches('/').to_string(),
            model,
        }
    }

    pub fn with_model(mut self, model: String) -> Self {
        self.model = model;
        self
    }
}

#[async_trait]
impl AiProvider for OllamaProvider {
    async fn generate_content(&self, prompt: &str) -> Result<String, ProviderError> {
        let config = GenerationConfig::default();
        self.generate_content_with_config(prompt, &config).await
    }

    async fn generate_content_with_config(
        &self,
        prompt: &str,
        config: &GenerationConfig,
    ) -> Result<String, ProviderError> {
        let url = format!("{}/api/generate", self.base_url);

        let request = OllamaGenerateRequest {
            model: self.model.clone(),
            prompt: prompt.to_string(),
            stream: false,
            options: OllamaOptions {
                temperature: config.temperature,
                top_p: config.top_p,
                num_predict: config.max_output_tokens as i32,
            },
        };

        let response = self.client.post(&url).json(&request).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(ProviderError::ApiError {
                message: format!("HTTP {}: {}", status, error_text),
            });
        }

        let response_body: OllamaGenerateResponse = response.json().await?;

        if !response_body.done {
            return Err(ProviderError::ApiError {
                message: "Incomplete response from Ollama".to_string(),
            });
        }

        Ok(response_body.response)
    }
}