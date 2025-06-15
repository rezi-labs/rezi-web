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
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Option<Usage>,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub index: u32,
    pub message: Option<Message>,
    pub delta: Option<Delta>,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Delta {
    pub role: Option<String>,
    pub content: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Debug)]
pub enum LlmError {
    RequestError(reqwest::Error),
    AuthError(String),
    ParseError(String),
}

impl From<reqwest::Error> for LlmError {
    fn from(err: reqwest::Error) -> Self {
        LlmError::RequestError(err)
    }
}

pub async fn chat_completion(messages: Vec<Message>) -> Result<String, LlmError> {
    let api_token = env::var("CHUTES_API_TOKEN").map_err(|_| {
        LlmError::AuthError("CHUTES_API_TOKEN environment variable not set".to_string())
    })?;

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
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(LlmError::RequestError(reqwest::Error::from(
            reqwest::Client::new().get("").send().await.unwrap_err(),
        )));
    }

    let completion_response: ChatCompletionResponse = response
        .json()
        .await
        .map_err(|e| LlmError::ParseError(format!("Failed to parse response: {}", e)))?;

    if let Some(choice) = completion_response.choices.first() {
        if let Some(message) = &choice.message {
            Ok(message.content.clone())
        } else {
            Err(LlmError::ParseError("No message content found".to_string()))
        }
    } else {
        Err(LlmError::ParseError(
            "No choices found in response".to_string(),
        ))
    }
}

pub async fn chat_completion_streaming(
    messages: Vec<Message>,
    mut callback: impl FnMut(String),
) -> Result<String, LlmError> {
    let api_token = env::var("CHUTES_API_TOKEN").map_err(|_| {
        LlmError::AuthError("CHUTES_API_TOKEN environment variable not set".to_string())
    })?;

    let client = Client::new();

    let request_body = ChatCompletionRequest {
        model: "deepseek-ai/DeepSeek-V3-0324".to_string(),
        messages,
        stream: true,
        max_tokens: 1024,
        temperature: 0.7,
    };

    let response = client
        .post("https://llm.chutes.ai/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_token))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(LlmError::ParseError(format!(
            "API error {}: {}",
            status, error_text
        )));
    }

    let mut full_content = String::new();
    let mut stream = response.bytes_stream();

    use futures_util::StreamExt;

    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result?;
        let chunk_str = String::from_utf8_lossy(&chunk);

        // Parse SSE format (data: {...})
        for line in chunk_str.lines() {
            if line.starts_with("data: ") {
                let json_str = &line[6..]; // Remove "data: " prefix

                if json_str == "[DONE]" {
                    break;
                }

                if let Ok(chunk_response) = serde_json::from_str::<ChatCompletionResponse>(json_str)
                {
                    if let Some(choice) = chunk_response.choices.first() {
                        if let Some(delta) = &choice.delta {
                            if let Some(content) = &delta.content {
                                full_content.push_str(content);
                                callback(content.to_owned());
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(full_content)
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
