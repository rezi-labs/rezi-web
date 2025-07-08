use std::{
    str::FromStr,
    sync::{Arc, Mutex},
};

use chrono::{DateTime, Utc};
use libsql_client::{Row, Statement};
use log::{error, info};

use crate::{
    config::Server,
    routes::{ChatMessage, Item},
};

pub type DBClient = Arc<Mutex<libsql_client::Client>>;

pub async fn create_client(url: String, token: Option<String>) -> libsql_client::Client {
    libsql_client::Client::from_config(libsql_client::Config {
        url: url::Url::parse(&url).unwrap(),
        auth_token: token,
    })
    .await
    .unwrap()
}

#[allow(clippy::await_holding_lock)]
pub async fn migrations(client: &DBClient, config: &Server) {
    let non_locked = &client.clone();
    let client = client.lock().unwrap();

    // Run base table migrations first
    let items_sql = include_str!("../migrations/items.sql");
    client
        .execute(items_sql)
        .await
        .expect("items migration failed");

    let chatmessages_sql = include_str!("../migrations/chatmessages.sql");
    client
        .execute(chatmessages_sql)
        .await
        .expect("chatmessages migration failed");

    // Run index migrations
    let idx_todos_completed_sql = include_str!("../migrations/idx_todos_completed.sql");
    client
        .execute(idx_todos_completed_sql)
        .await
        .expect("idx_todos_completed migration failed");

    let idx_todos_created_at_sql = include_str!("../migrations/idx_todos_created_at.sql");
    client
        .execute(idx_todos_created_at_sql)
        .await
        .expect("idx_todos_created_at migration failed");

    let idx_chat_messages_sender_sql = include_str!("../migrations/idx_chat_messages_sender.sql");
    client
        .execute(idx_chat_messages_sender_sql)
        .await
        .expect("idx_chat_messages_sender migration failed");

    let idx_chat_messages_timestamp_sql =
        include_str!("../migrations/idx_chat_messages_timestamp.sql");
    client
        .execute(idx_chat_messages_timestamp_sql)
        .await
        .expect("idx_chat_messages_timestamp migration failed");

    let idx_chat_messages_is_user_sql = include_str!("../migrations/idx_chat_messages_is_user.sql");
    client
        .execute(idx_chat_messages_is_user_sql)
        .await
        .expect("idx_chat_messages_is_user migration failed");

    // Run trigger migrations last
    let update_todos_timestamp_trigger_sql =
        include_str!("../migrations/update_todos_timestamp_trigger.sql");
    client
        .execute(update_todos_timestamp_trigger_sql)
        .await
        .expect("update_todos_timestamp_trigger migration failed");

    let witch_results_sql = include_str!("../migrations/witch_results.sql");
    client
        .execute(witch_results_sql)
        .await
        .expect("with_results migration failed");

    drop(client);

    // test client if in local
    if config.db_token().is_none() {
        let item = (0..9).map(|_| Item::random_item()).collect::<Vec<Item>>();
        create_items(non_locked, item, "test_user".to_string()).await;
    }
}

fn escape_sql_string(s: &str) -> String {
    s.replace("'", "''") // Escape single quotes by doubling them
}

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

#[allow(unused)]
pub struct WitchResult {
    pub id: u32,
    pub url: String,
    pub content: String,
    pub owner_id: String,
    pub timestamp: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[allow(unused)]
impl WitchResult {
    pub fn new(
        id: u32,
        url: String,
        content: String,
        owner_id: String,
        timestamp: DateTime<Utc>,
        created_at: DateTime<Utc>,
    ) -> Self {
        WitchResult {
            id,
            url,
            content,
            owner_id,
            timestamp,
            created_at,
        }
    }

    pub fn id(&self) -> &u32 {
        &self.id
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn owner_id(&self) -> &str {
        &self.owner_id
    }

    pub fn from_row(row: &Row) -> Result<WitchResult, String> {
        let id: u32 = row.try_get(0).unwrap();
        let url: &str = row.try_get(1).unwrap();
        let content: &str = row.try_get(2).unwrap();
        let owner_id: &str = row.try_get(3).unwrap();
        let timestamp: &str = row.try_get(4).unwrap();
        let timestamp: DateTime<Utc> = DateTime::from_str(timestamp).unwrap();

        Ok(WitchResult::new(
            id,
            url.to_string(),
            content.to_string(),
            owner_id.to_string(),
            timestamp,
            timestamp,
        ))
    }
}
#[allow(clippy::await_holding_lock)]
pub async fn add_witch_result(
    client: &DBClient,
    url: &str,
    content: &str,
    owner_id: &str,
) -> Result<u32, String> {
    let client = client.lock().unwrap();

    let stmt = libsql_client::Statement::with_args(
        "
            INSERT INTO witch_results (url, content, owner_id, timestamp, created_at)
             VALUES (?, ?, ?, ?, CURRENT_TIMESTAMP) RETURNING id
            ",
        &[url, content, owner_id, &Utc::now().to_string()],
    );

    let res = client.execute(stmt).await.unwrap();
    let row = res.rows.first().ok_or("No row found")?;
    let id: u32 = row.try_get(0).unwrap();

    drop(client);
    Ok(id)
}
#[allow(clippy::await_holding_lock)]
pub async fn get_single_witch_result(client: &DBClient, id: &u32) -> Result<WitchResult, String> {
    let client = client.lock().unwrap();

    let stmt = libsql_client::Statement::with_args(
        "
            SELECT id, url, content, owner_id, timestamp, created_at
             FROM witch_results
             WHERE id = ?
            ",
        &[id.to_string()],
    );

    let res = client.execute(stmt).await.unwrap();

    drop(client);
    let row = res.rows.first().ok_or("No row found")?;
    WitchResult::from_row(row).map_err(|e| e.to_string())
}

#[allow(clippy::await_holding_lock)]
pub async fn get_witch_results(client: &DBClient, user_id: &str) -> Vec<WitchResult> {
    let client = client.lock().unwrap();

    let stmt = libsql_client::Statement::with_args(
        "
            SELECT id, url, content, owner_id, timestamp, created_at
             FROM witch_results
             WHERE owner_id = ?
             ORDER BY timestamp ASC
            ",
        &[user_id],
    );

    let res = client.execute(stmt).await.unwrap();

    drop(client);
    let rows: Vec<WitchResult> = res
        .rows
        .iter()
        .filter_map(|r| WitchResult::from_row(r).ok())
        .collect();
    rows
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

#[allow(clippy::await_holding_lock)]
pub async fn get_items(client: &DBClient, owner_id: String) -> Vec<Item> {
    let client = client.lock().unwrap();

    let stmt = libsql_client::Statement::with_args(
        "
            SELECT id, task, completed
            FROM items
            WHERE owner_id = ?
            ",
        &[owner_id],
    );

    let res = client.execute(stmt).await.unwrap();

    drop(client);

    let rows: Vec<Item> = res
        .rows
        .iter()
        .filter_map(|r| Item::from_row(r).ok())
        .collect();
    rows
}

#[allow(clippy::await_holding_lock)]
pub async fn create_items(client: &DBClient, items: Vec<Item>, user_id: String) {
    if items.is_empty() {
        return;
    }

    let client = client.lock().unwrap();

    // Build bulk insert SQL with multiple VALUES clauses
    let mut values_clauses = Vec::new();
    for item in &items {
        let completed = bool_to_int(item.completed);
        let escaped_task = escape_sql_string(&item.task);
        let value_clause = format!(
            "({}, '{}', '{}', '{}')",
            item.id, user_id, escaped_task, completed
        );
        values_clauses.push(value_clause);
    }

    let statement = format!(
        r#"INSERT INTO items (id, owner_id, task, completed) VALUES {};"#,
        values_clauses.join(", ")
    );

    info!("Bulk inserting {} items", items.len());

    let st = Statement::new(statement);
    let res = client.execute(st).await;

    drop(client);

    match res {
        Ok(s) => info!("Successfully inserted {} items: {:?}", items.len(), s),
        Err(e) => error!("Failed to bulk insert items: {e}"),
    }
}

#[allow(clippy::await_holding_lock)]
pub async fn create_item(client: &DBClient, item: Item, owner_id: String) {
    let client = client.lock().unwrap();

    info!("{item:?}");

    let completed = bool_to_int(item.completed);

    let statement = format!(
        r#"INSERT INTO items
    (id, owner_id, task, completed)
    VALUES({}, '{}', '{}', '{completed}');"#,
        item.id,
        owner_id,
        escape_sql_string(&item.task),
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
pub async fn delete_item(client: &DBClient, item_id: i64, owner_id: String) {
    let client = client.lock().unwrap();

    let statement = format!(
        r#"DELETE FROM items
    WHERE id = {item_id} AND owner_id = {owner_id};"#,
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
pub async fn toggle_item(
    client: &DBClient,
    item_id: i64,
    owner_id: String,
) -> Result<Item, String> {
    let locked_client = client.lock().unwrap();
    info!("db toggle: {item_id}");

    let statement = format!(
        r#"SELECT id, task, completed
    FROM items
    WHERE id = {item_id} AND owner_id = {owner_id};"#,
    );

    let st = Statement::new(statement);

    let res = locked_client.execute(st).await.unwrap();
    drop(locked_client);

    let rows: Vec<Item> = res
        .rows
        .iter()
        .filter_map(|r| Item::from_row(r).ok())
        .collect();

    let old = rows
        .first()
        .cloned()
        .ok_or_else(|| "Item not found".to_string())
        .unwrap();

    info!("old: {old:?}");

    let locked_client = client.lock().unwrap();

    let completed = bool_to_int(!old.completed);
    let statement = format!(
        r#"UPDATE items
    SET completed = {completed}
    WHERE id = {item_id};"#,
    );

    let st = Statement::new(statement);

    let _res = locked_client.execute(st).await;

    drop(locked_client);
    get_item(client, item_id, owner_id).await
}

#[allow(clippy::await_holding_lock)]
pub async fn get_item(client: &DBClient, item_id: i64, owner_id: String) -> Result<Item, String> {
    let client = client.lock().unwrap();

    let statement = format!(
        r#"SELECT id, task, completed
    FROM items
    WHERE id = {item_id} AND owner_id = {owner_id};"#,
    );

    let st = Statement::new(statement);

    let res = client.execute(st).await.unwrap();
    drop(client);

    let rows: Vec<Item> = res
        .rows
        .iter()
        .filter_map(|r| Item::from_row(r).ok())
        .collect();

    rows.first()
        .cloned()
        .ok_or_else(|| "Item not found".to_string())
}

pub fn bool_to_int(b: bool) -> i64 {
    if b { 1 } else { 0 }
}

pub fn int_to_bool(i: i64) -> bool {
    i == 1
}
