use libsql_client::{Row, Statement};
use log::{error, info};
use serde::{Deserialize, Serialize};

use crate::{
    database::{DBClient, escape_sql_string},
    routes::random_id,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: u32,
    pub task: String,
    pub completed: bool,
}

impl Item {
    pub fn random_item() -> Item {
        Item {
            id: random_id(),
            task: String::from("Random Task"),
            completed: false,
        }
    }

    pub fn from_row(row: &Row) -> Result<Item, String> {
        let Ok(id) = row.try_get::<u32>(0) else {
            let err = format!("Item::from_row {row:?}");
            return Err(err);
        };

        let Ok(task) = row.try_get::<&str>(1) else {
            let err = format!("Item::from_row {row:?}");
            return Err(err);
        };

        let Ok(completed) = row.try_get::<i64>(2) else {
            let err = format!("Item::from_row {row:?}");
            return Err(err);
        };

        let completed: bool = int_to_bool(completed);

        Ok(Item {
            id,
            task: task.to_string(),
            completed,
        })
    }
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

#[allow(clippy::await_holding_lock)]
pub async fn update_item(
    client: &DBClient,
    item_id: i64,
    new_task: String,
    owner_id: String,
) -> Result<Item, String> {
    let locked_client = client.lock().unwrap();

    let statement = format!(
        r#"UPDATE items
         SET task = '{}'
         WHERE id = {} AND owner_id = '{}';"#,
        escape_sql_string(&new_task),
        item_id,
        owner_id
    );

    let st = Statement::new(statement);

    let res = locked_client.execute(st).await;
    drop(locked_client);

    match res {
        Ok(_) => get_item(client, item_id, owner_id).await,
        Err(e) => {
            error!("Failed to update item: {e}");
            Err(format!("Failed to update item: {e}"))
        }
    }
}
