use std::sync::{Arc, Mutex};

use libsql_client::Statement;
use log::{error, info};

use crate::routes::{ChatMessage, Item};

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
pub async fn migrations(client: &DBClient) {
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

    drop(client);
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
    (id, content, sender, "timestamp", is_user, created_at)
    VALUES({}, '{}', '{}', '{}', '{}', CURRENT_TIMESTAMP);"#,
        message.id,
        escape_sql_string(&message.content),
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
pub async fn get_messages(client: &DBClient, _sender: &str) -> Vec<ChatMessage> {
    let client = client.lock().unwrap();

    let stmt = libsql_client::Statement::new(
        "
            SELECT id, content, sender, timestamp, is_user, created_at
             FROM chat_messages
             ORDER BY timestamp ASC
            ",
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
pub async fn get_items(client: &DBClient) -> Vec<Item> {
    let client = client.lock().unwrap();

    let stmt = libsql_client::Statement::new(
        "
            SELECT id, task, completed
            FROM items
            ",
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
pub async fn create_items(client: &DBClient, items: Vec<Item>) {
    if items.is_empty() {
        return;
    }

    let client = client.lock().unwrap();

    // Build bulk insert SQL with multiple VALUES clauses
    let mut values_clauses = Vec::new();
    for item in &items {
        let completed = bool_to_int(item.completed);
        let escaped_task = escape_sql_string(&item.task);
        let value_clause = format!("({}, '{}', '{}')", item.id, escaped_task, completed);
        values_clauses.push(value_clause);
    }

    let statement = format!(
        r#"INSERT INTO items (id, task, completed) VALUES {};"#,
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
pub async fn create_item(client: &DBClient, item: Item) {
    let client = client.lock().unwrap();

    info!("{item:?}");

    let completed = bool_to_int(item.completed);

    let statement = format!(
        r#"INSERT INTO items
    (id, task, completed)
    VALUES({}, '{}', '{completed}');"#,
        item.id,
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
pub async fn delete_item(client: &DBClient, item_id: i64) {
    let client = client.lock().unwrap();

    let statement = format!(
        r#"DELETE FROM items
    WHERE id = {item_id};"#,
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
pub async fn toggle_item(client: &DBClient, item_id: i64) -> Result<Item, String> {
    let locked_client = client.lock().unwrap();
    info!("db toggle: {item_id}");

    let statement = format!(
        r#"SELECT id, task, completed
    FROM items
    WHERE id = {item_id};"#,
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
    get_item(client, item_id).await
}

#[allow(clippy::await_holding_lock)]
pub async fn get_item(client: &DBClient, item_id: i64) -> Result<Item, String> {
    let client = client.lock().unwrap();

    let statement = format!(
        r#"SELECT id, task, completed
    FROM items
    WHERE id = {item_id};"#,
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
