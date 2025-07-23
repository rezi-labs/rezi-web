use libsql_client::Statement;
use log::{error, info};

use crate::{
    database::{DBClient, escape_sql_string},
    routes::ChatMessage,
};

#[allow(clippy::await_holding_lock)]
pub async fn save_message(client: &DBClient, message: ChatMessage) {
    let client = client.lock().unwrap();

    info!("{message:?}");

    let statement = format!(
        r#"INSERT INTO chat_messages
    (id, content, ai_response, sender, "timestamp", is_user, created_at)
    VALUES({}, '{}', '{}', '{}', '{}', '{}', CURRENT_TIMESTAMP);"#,
        message.id,
        escape_sql_string(&message.content),
        escape_sql_string(&message.ai_response),
        message.sender,
        message.timestamp,
        message.is_user
    );

    let st = Statement::new(statement);

    let res = client.execute(st).await;
    drop(client);
    match res {
        Ok(s) => info!("{s:?}"),
        Err(e) => error!("{e}"),
    }
}

#[allow(clippy::await_holding_lock)]
pub async fn get_messages(client: &DBClient, user_id: &str) -> Vec<ChatMessage> {
    let client = client.lock().unwrap();

    let stmt = libsql_client::Statement::with_args(
        "
            SELECT id, content, ai_response, sender, timestamp, is_user, created_at
             FROM chat_messages
             WHERE sender = ?
             ORDER BY timestamp ASC
            ",
        &[user_id],
    );

    let res = client.execute(stmt).await.unwrap();

    drop(client);
    let rows: Vec<ChatMessage> = res
        .rows
        .iter()
        .filter_map(|r| ChatMessage::from_row(r).ok())
        .collect();
    rows
}
