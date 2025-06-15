use std::sync::{Arc, Mutex};

use libsql_client::Statement;
use log::{error, info};

use crate::routes::{ChatMessage, Item};

pub type DBClient = Arc<Mutex<libsql_client::Client>>;

pub async fn create_client(url: String) -> libsql_client::Client {
    libsql_client::Client::from_config(libsql_client::Config {
        url: url::Url::parse(&url).unwrap(),
        auth_token: None,
    })
    .await
    .unwrap()
}

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

pub async fn save_message(client: &DBClient, message: ChatMessage) {
    let client = client.lock().unwrap();
    
    info!("{:?}", message);

    let statement = format!(
        r#"INSERT INTO chat_messages
    (id, content, sender, "timestamp", is_user, created_at)
    VALUES({}, '{}', '{}', '{}', '{}', CURRENT_TIMESTAMP);"#,
        message.id, escape_sql_string(&message.content), message.sender, message.timestamp, message.is_user
    );
    
    
    let st = Statement::new(statement);

    let res = client.execute(st).await;
drop(client);
    match res {
        Ok(s) => info!("{:?}", s),
        Err(e) => error!("{}", e.to_string()),
    }
}

pub async fn get_messages(client: &DBClient, sender: &str) -> Vec<ChatMessage> {
    let client = client.lock().unwrap();

    let stmt = libsql_client::Statement::new(
        "
            SELECT id, content, sender, timestamp, is_user, created_at 
             FROM chat_messages
             ORDER BY timestamp ASC
            "
    );

    let res = client.execute(stmt).await.unwrap();

    let rows: Vec<ChatMessage> = res
        .rows
        .iter()
        .filter_map(|r| ChatMessage::from_row(r).ok())
        .collect();
    rows
}

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

pub async fn create_item(client: &DBClient, item: Item) {
    let client = client.lock().unwrap();
    
    info!("{:?}", item);

    let statement = format!(
        r#"INSERT INTO items
    (id, task, completed)
    VALUES({}, '{}', '{}');"#,
        item.id, escape_sql_string(&item.task), item.completed
    );
    
    
    let st = Statement::new(statement);

    let res = client.execute(st).await;
    drop(client);
    match res {
        Ok(s) => info!("{:?}", s),
        Err(e) => error!("{}", e.to_string()),
    }
}