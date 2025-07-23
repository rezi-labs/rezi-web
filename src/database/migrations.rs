use crate::{
    config::Server,
    database::{
        DBClient,
        items::{self, Item},
    },
};

#[allow(clippy::await_holding_lock)]
pub async fn run(client: &DBClient, config: &Server) {
    let non_locked = &client.clone();
    let client = client.lock().unwrap();

    // Run base table migrations first
    let items_sql = include_str!("../../migrations/items.sql");
    client
        .execute(items_sql)
        .await
        .expect("items migration failed");

    let chatmessages_sql = include_str!("../../migrations/chatmessages.sql");
    client
        .execute(chatmessages_sql)
        .await
        .expect("chatmessages migration failed");

    // Run index migrations
    let idx_todos_completed_sql = include_str!("../../migrations/idx_todos_completed.sql");
    client
        .execute(idx_todos_completed_sql)
        .await
        .expect("idx_todos_completed migration failed");

    let idx_todos_created_at_sql = include_str!("../../migrations/idx_todos_created_at.sql");
    client
        .execute(idx_todos_created_at_sql)
        .await
        .expect("idx_todos_created_at migration failed");

    let idx_chat_messages_sender_sql =
        include_str!("../../migrations/idx_chat_messages_sender.sql");
    client
        .execute(idx_chat_messages_sender_sql)
        .await
        .expect("idx_chat_messages_sender migration failed");

    let idx_chat_messages_timestamp_sql =
        include_str!("../../migrations/idx_chat_messages_timestamp.sql");
    client
        .execute(idx_chat_messages_timestamp_sql)
        .await
        .expect("idx_chat_messages_timestamp migration failed");

    let idx_chat_messages_is_user_sql =
        include_str!("../../migrations/idx_chat_messages_is_user.sql");
    client
        .execute(idx_chat_messages_is_user_sql)
        .await
        .expect("idx_chat_messages_is_user migration failed");

    // Run trigger migrations last
    let update_todos_timestamp_trigger_sql =
        include_str!("../../migrations/update_todos_timestamp_trigger.sql");
    client
        .execute(update_todos_timestamp_trigger_sql)
        .await
        .expect("update_todos_timestamp_trigger migration failed");

    let witch_results_sql = include_str!("../../migrations/recipes.sql");
    client
        .execute(witch_results_sql)
        .await
        .expect("with_results migration failed");

    drop(client);

    // test client if in local
    if config.db_token().is_none() {
        let item = (0..9).map(|_| Item::random_item()).collect::<Vec<Item>>();
        items::create_items(non_locked, item, "test_user".to_string()).await;
    }
}
