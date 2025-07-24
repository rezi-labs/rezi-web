use crate::{config::Server, database::DBClient2};

#[allow(clippy::await_holding_lock)]
pub async fn run(client: &DBClient2, config: &Server) {
    let client = client.lock().unwrap();

    // Run base table migrations first
    let items_sql = include_str!("../../migrations/items.sql");

    client
        .get_connection()
        .execute_batch(items_sql)
        .await
        .expect("items migration failed");

    let messages_sql = include_str!("../../migrations/messages.sql");
    client
        .get_connection()
        .execute_batch(messages_sql)
        .await
        .expect("messages migration failed");

    let recipes_sql = include_str!("../../migrations/recipes.sql");
    client
        .get_connection()
        .execute_batch(recipes_sql)
        .await
        .expect("recipes migration failed");

    drop(client);
}
