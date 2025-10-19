use crate::providers::{AiProvider, GenerationConfig, ProviderError};
use std::sync::Arc;

#[derive(Clone)]
pub struct AiClient {
    provider: Arc<dyn AiProvider>,
}

impl std::fmt::Debug for AiClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AiClient")
            .field("provider", &"<dyn AiProvider>")
            .finish()
    }
}

impl AiClient {
    pub fn new(provider: Box<dyn AiProvider>) -> Self {
        Self {
            provider: Arc::from(provider),
        }
    }

    pub async fn generate_content(&self, prompt: &str) -> Result<String, ProviderError> {
        self.provider.generate_content(prompt).await
    }

    pub async fn generate_content_with_config(
        &self,
        prompt: &str,
        config: &GenerationConfig,
    ) -> Result<String, ProviderError> {
        self.provider.generate_content_with_config(prompt, config).await
    }
}