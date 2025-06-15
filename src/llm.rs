use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub stream: bool,
    pub max_tokens: u32,
    pub temperature: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct ChatCompletionResponse {
    pub choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub message: Option<Message>,
}

#[derive(Debug)]
pub enum LlmError {
    Request(String),
    Auth(String),
    Parse(String),
}

pub async fn chat_completion(messages: Vec<Message>) -> Result<String, LlmError> {
    let api_token = env::var("CHUTES_API_TOKEN")
        .map_err(|_| LlmError::Auth("CHUTES_API_TOKEN environment variable not set".to_string()))?;

    let client = Client::new();

    let request_body = ChatCompletionRequest {
        model: "deepseek-ai/DeepSeek-V3-0324".to_string(),
        messages,
        stream: false, // Set to false for non-streaming response
        max_tokens: 1024,
        temperature: 0.7,
    };

    let response = client
        .post("https://llm.chutes.ai/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_token))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await
        .map_err(|e| LlmError::Request(e.to_string()))?;

    if !response.status().is_success() {
        let status = response.status();
        let text = response
            .text()
            .await
            .map_or("Unknown error".to_string(), |i| i);

        let f = format!("status: {} text: {}", status.as_str(), text);

        return Err(LlmError::Request(f));
    }

    let completion_response: ChatCompletionResponse = response
        .json()
        .await
        .map_err(|e| LlmError::Parse(format!("Failed to parse response: {}", e)))?;

    if let Some(choice) = completion_response.choices.first() {
        if let Some(message) = &choice.message {
            Ok(message.content.clone())
        } else {
            Err(LlmError::Parse("No message content found".to_string()))
        }
    } else {
        Err(LlmError::Parse("No choices found in response".to_string()))
    }
}

pub async fn simple_chat(user_message: &str) -> Result<String, LlmError> {
    let messages = vec![Message {
        role: "user".to_string(),
        content: user_message.to_string(),
    }];

    chat_completion(messages).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_simple_chat() {
        // This test requires CHUTES_API_TOKEN to be set
        if env::var("CHUTES_API_TOKEN").is_ok() {
            let result = simple_chat("Tell me a short joke").await;
            match result {
                Ok(response) => {
                    println!("Response: {}", response);
                    assert!(!response.is_empty());
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            }
        }
    }
}
