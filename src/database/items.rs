use libsql_orm::{Filter, FilterOperator, Model};
use serde::{Deserialize, Serialize};

use crate::database::DBClient2;

#[derive(Model, Debug, Clone, Serialize, Deserialize)]
#[table_name("items")]
pub struct Item {
    pub id: std::option::Option<i64>,
    pub owner_id: String,
    pub task: String,
    pub completed: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Item {
    pub fn random_item() -> Item {
        Item {
            id: None,
            owner_id: String::from("user1"),
            task: String::from("Random Task"),
            completed: false,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }

    pub fn id(&self) -> i64 {
        self.id.unwrap()
    }

    pub fn update_task(&mut self, task: &str) {
        self.task = task.to_string()
    }
    pub fn toggle(&mut self) {
        self.completed = !self.completed
    }
}

#[allow(clippy::await_holding_lock)]
pub async fn get_items(client: &DBClient2, owner_id: String) -> Vec<Item> {
    let db = client.lock().unwrap();

    Item::find_where(
        FilterOperator::Single(Filter::eq("owner_id".to_string(), owner_id)),
        &db,
    )
    .await
    .unwrap_or_default()
}

#[allow(clippy::await_holding_lock)]
pub async fn create_items(client: &DBClient2, items: Vec<Item>) {
    if items.is_empty() {
        return;
    }

    let client = client.lock().unwrap();
    Item::bulk_create(items.as_slice(), &client).await;

    drop(client);
}

#[allow(clippy::await_holding_lock)]
pub async fn create_item(client: &DBClient2, item: Item) {
    let client = client.lock().unwrap();

    let res = item.create(&client).await;
    match res {
        Ok(item) => log::info!("created item {}", item.id()),
        Err(err) => log::error!("{err:?}"),
    }
}
#[allow(clippy::await_holding_lock)]
pub async fn delete_item(client: &DBClient2, item_id: i64, owner_id: String) {
    let db = client.lock().unwrap();

    let res = Item::find_by_id(item_id, &db)
        .await
        .unwrap()
        .unwrap()
        .delete(&db)
        .await;
    match res {
        Ok(_) => todo!(),
        Err(err) => log::error!("{err:?}"),
    }
}

#[allow(clippy::await_holding_lock)]
pub async fn toggle_item(
    client: &DBClient2,
    item_id: i64,
    owner_id: String,
) -> Result<Item, String> {
    let db = client.lock().unwrap();

    let mut item = Item::find_by_id(item_id, &db).await.unwrap().unwrap();

    if item.owner_id != owner_id {
        return Err("Nope".to_string());
    }

    item.toggle();

    let _ = item.upsert(&["completed"], &db).await;

    get_item(client, item_id, owner_id).await
}

#[allow(clippy::await_holding_lock)]
pub async fn get_item(client: &DBClient2, item_id: i64, owner_id: String) -> Result<Item, String> {
    let db = client.lock().unwrap();

    let item = Item::find_by_id(item_id, &db)
        .await
        .map_err(|e| e.to_string());

    let Ok(item) = item else {
        return Err("Nope not working".to_string());
    };

    let Some(item) = item else {
        log::error!("can not find item: {item_id}");
        return Err("Nope not found".to_string());
    };

    if item.owner_id != owner_id {
        return Err("Nope".to_string());
    }

    Ok(item)
}

#[allow(clippy::await_holding_lock)]
pub async fn update_item(
    client: &DBClient2,
    item_id: i64,
    new_task: String,
    owner_id: String,
) -> Result<Item, String> {
    let db = client.lock().unwrap();

    let mut item = Item::find_by_id(item_id, &db).await.unwrap().unwrap();

    if item.owner_id != owner_id {
        return Err("Nope".to_string());
    };

    item.update_task(&new_task);

    item.upsert(&["task"], &db).await.map_err(|e| e.to_string())
}
