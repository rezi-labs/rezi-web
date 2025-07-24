use crate::database::DBClient2;

pub async fn run(client: &DBClient2) {
    log::info!("Starting database migrations...");

    // Run base table migrations first
    let items_sql = include_str!("../../migrations/items.sql");
    {
        let client = client.lock().unwrap();
        client
            .get_connection()
            .execute_batch(items_sql)
            .await
            .expect("items migration failed");
    }
    log::info!("Items table migration completed");

    let messages_sql = include_str!("../../migrations/messages.sql");
    {
        let client = client.lock().unwrap();
        client
            .get_connection()
            .execute_batch(messages_sql)
            .await
            .expect("messages migration failed");
    }
    log::info!("Messages table migration completed");

    let recipes_sql = include_str!("../../migrations/recipes.sql");
    {
        let client = client.lock().unwrap();
        client
            .get_connection()
            .execute_batch(recipes_sql)
            .await
            .expect("recipes migration failed");
    }
    log::info!("Recipes table migration completed");

    // Run index migrations
    let items_indexes_sql = include_str!("../../migrations/items_indexes.sql");
    {
        let client = client.lock().unwrap();
        client
            .get_connection()
            .execute_batch(items_indexes_sql)
            .await
            .expect("items indexes migration failed");
    }
    log::info!("Items indexes migration completed");

    let messages_indexes_sql = include_str!("../../migrations/messages_indexes.sql");
    {
        let client = client.lock().unwrap();
        client
            .get_connection()
            .execute_batch(messages_indexes_sql)
            .await
            .expect("messages indexes migration failed");
    }
    log::info!("Messages indexes migration completed");

    let recipes_indexes_sql = include_str!("../../migrations/recipes_indexes.sql");
    {
        let client = client.lock().unwrap();
        client
            .get_connection()
            .execute_batch(recipes_indexes_sql)
            .await
            .expect("recipes indexes migration failed");
    }
    log::info!("Recipes indexes migration completed");

    log::info!("All database migrations completed successfully");
}
