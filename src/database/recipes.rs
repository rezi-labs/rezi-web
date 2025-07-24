use libsql_orm::Model;
use serde::{Deserialize, Serialize};

#[allow(unused)]
#[derive(Model, Debug, Clone, Serialize, Deserialize)]
#[table_name("recipes")]
pub struct Recipe {
    pub id: std::option::Option<i64>,
    pub owner_id: String,
    pub title: Option<String>,
    pub url: Option<String>,
    pub content: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[allow(unused)]
impl Recipe {
    pub fn new(
        id: Option<i64>,
        owner_id: String,
        title: Option<String>,
        url: Option<String>,
        content: String,
    ) -> Self {
        Recipe {
            id,
            owner_id,
            title,
            url,
            content,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }

    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    pub fn url(&self) -> Option<&str> {
        self.url.as_deref()
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn owner_id(&self) -> &str {
        &self.owner_id
    }

    pub fn id(&self) -> i64 {
        self.id.unwrap_or(0)
    }

    pub fn examples() -> Vec<Recipe> {
        vec![
            Recipe::new(
                None,
                "1".to_string(),
                Some("Pumpkin Soup".to_string()),
                None,
                "Boil pumpkin and onions".to_string(),
            ),
            Recipe::new(
                None,
                "2".to_string(),
                Some("Chocolate Cake".to_string()),
                None,
                "Mix flour, sugar, eggs, and chocolate".to_string(),
            ),
        ]
    }
}
