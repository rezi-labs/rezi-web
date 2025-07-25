use libsql_orm::{Filter, FilterOperator, Model};
use serde::{Deserialize, Serialize};

use crate::database::DBClient;

#[derive(Model, Debug, Clone, Serialize, Deserialize)]
#[table_name("items")]
pub struct Item {
    pub id: std::option::Option<i64>,
    pub owner_id: String,
    pub task: String,
    pub completed: u16,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Item {
    pub fn id(&self) -> i64 {
        self.id.unwrap()
    }

    pub fn update_task(&mut self, task: &str) {
        self.task = task.to_string()
    }
    pub fn toggle(&mut self) {
        if self.completed == 0 {
            self.completed = 1
        } else {
            self.completed = 0
        }
    }

    pub fn completed(&self) -> bool {
        self.completed == 1
    }

    pub fn owner_id(&self) -> String {
        self.owner_id.to_string()
    }
}

pub async fn get_items(client: &DBClient, owner_id: String) -> Result<Vec<Item>, String> {
    log::info!("getting items for owner: {owner_id}");

    let db = super::unlock_client(client).await;
    let items = Item::find_where(
        FilterOperator::Single(Filter::eq("owner_id".to_string(), owner_id.clone())),
        &db,
    )
    .await;
    drop(db);

    match items {
        Ok(items) => {
            log::info!("found {} items for owner: {}", items.len(), owner_id);
            Ok(items)
        }
        Err(err) => {
            log::error!("Error getting items: {err}");
            Err("Could not get items".to_string())
        }
    }
}

pub async fn create_items(client: &DBClient, items: Vec<Item>) {
    if items.is_empty() {
        return;
    }

    let client = super::unlock_client(client).await;
    let result = Item::bulk_create(items.as_slice(), &client).await;
    match result {
        Ok(_) => log::info!("created items"),
        Err(err) => log::error!("could not create items: {err}"),
    }
}

pub async fn create_item(client: &DBClient, item: Item) -> Result<Item, String> {
    let db = super::unlock_client(client).await;

    let res = Item::create(&item, &db).await;
    drop(db);

    match res {
        Ok(created_item) => {
            log::info!("created item {}", created_item.id());
            Ok(created_item)
        }
        Err(err) => {
            log::error!("{err:?}");
            Err("Could not create item".to_string())
        }
    }
}
pub async fn delete_item(client: &DBClient, item_id: i64, owner_id: String) {
    let db = super::unlock_client(client).await;
    let item_result = Item::find_by_id(item_id, &db).await;

    match item_result {
        Ok(Some(item)) => {
            if item.owner_id() != owner_id {
                log::error!("Unauthorized delete attempt for item {item_id}");
                drop(db);
                return;
            }

            let delete_result = item.delete(&db).await;
            drop(db);

            match delete_result {
                Ok(_) => log::info!("Successfully deleted item {item_id}"),
                Err(err) => log::error!("Failed to delete item {item_id}: {err:?}"),
            }
        }
        Ok(None) => {
            log::error!("Item {item_id} not found");
            drop(db);
        }
        Err(err) => {
            log::error!("Error finding item {item_id}: {err:?}");
            drop(db);
        }
    }
}

pub async fn toggle_item(
    client: &DBClient,
    item_id: i64,
    owner_id: String,
) -> Result<Item, String> {
    let db = super::unlock_client(client).await;
    let item_result = Item::find_by_id(item_id, &db).await;

    let mut item = match item_result {
        Ok(Some(item)) => item,
        Ok(None) => {
            drop(db);
            return Err("Item not found".to_string());
        }
        Err(err) => {
            drop(db);
            log::error!("Error finding item: {err:?}");
            return Err("Database error".to_string());
        }
    };

    if item.owner_id() != owner_id {
        drop(db);
        return Err("Unauthorized".to_string());
    }

    item.toggle();

    let update_result = item.update(&db).await;
    drop(db);

    match update_result {
        Ok(updated_item) => {
            log::info!("toggled item {}", updated_item.id());
            Ok(updated_item)
        }
        Err(err) => {
            log::error!("could not toggle item: {err}");
            Err("Failed to update item".to_string())
        }
    }
}

pub async fn get_item(client: &DBClient, item_id: i64, owner_id: String) -> Result<Item, String> {
    let db = super::unlock_client(client).await;
    let item_result = Item::find_by_id(item_id, &db).await;
    drop(db);

    match item_result {
        Ok(Some(item)) => {
            if item.owner_id() != owner_id {
                return Err("Unauthorized".to_string());
            }
            log::info!("found item {}", item.id());
            Ok(item)
        }
        Ok(None) => {
            log::error!("item not found: {item_id}");
            Err("Item not found".to_string())
        }
        Err(err) => {
            log::error!("database error finding item {item_id}: {err}");
            Err("Database error".to_string())
        }
    }
}

pub async fn update_item(
    client: &DBClient,
    item_id: i64,
    new_task: String,
    owner_id: String,
) -> Result<Item, String> {
    let db = super::unlock_client(client).await;
    let item_result = Item::find_by_id(item_id, &db).await;

    let mut item = match item_result {
        Ok(Some(item)) => item,
        Ok(None) => {
            drop(db);
            return Err("Item not found".to_string());
        }
        Err(err) => {
            drop(db);
            log::error!("Error finding item: {err:?}");
            return Err("Database error".to_string());
        }
    };

    if item.owner_id() != owner_id {
        drop(db);
        return Err("Unauthorized".to_string());
    }

    item.update_task(&new_task);
    item.updated_at = chrono::Utc::now();

    let update_result = item.update(&db).await;
    drop(db);

    update_result.map_err(|e| e.to_string())
}
