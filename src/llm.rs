use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{
    database::{self, DBClient},
    routes::{Item, random_id},
};

#[derive(Debug)]
pub enum LlmError {
    Request(String),
    Auth(String),
    Parse(String),
}

#[derive(Debug, Serialize)]
pub struct Prompt {
    prompt: String,
}

#[derive(Debug, Deserialize)]
pub struct TaskList {
    list: Vec<String>,
}

pub async fn simple_chat(
    nest_api: &str,
    nest_api_key: &str,
    user_message: &str,
    db_client: &DBClient,
) -> Result<String, LlmError> {
    let client = Client::new();

    let prompt = Prompt {
        prompt: user_message.to_string(),
    };

    let full_url = format!("{}{}", nest_api, "/api/task");

    let response = client
        .post(full_url)
        .header("api-key", nest_api_key)
        .json(&prompt)
        .send()
        .await
        .map_err(|e| LlmError::Request(format!("Failed to send request: {e}")))?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        return Err(LlmError::Auth(format!(
            "API returned status {status}: {error_text}"
        )));
    }

    let task_list: TaskList = response
        .json()
        .await
        .map_err(|e| LlmError::Parse(format!("Failed to parse response: {e}")))?;

    let items: Vec<Item> = task_list
        .list
        .iter()
        .map(|t| Item {
            id: random_id(),
            task: t.clone(),
            completed: false,
        })
        .collect();

    database::create_items(db_client, items).await;

    let tasks_string = task_list.list.join("\n");

    let answer = format!("Created {tasks_string}");

    Ok(answer)
}
