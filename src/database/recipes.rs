use libsql_orm::Model;
use serde::{Deserialize, Serialize};


#[allow(unused)]
#[derive(Model, Debug, Clone, Serialize, Deserialize)]
#[table_name("recipes")]
pub struct Recipe {
    id: std::option::Option<i64>,
    owner_id: String,
    name: String,
    url: Option<String>,
    image_url: Option<String>,
    extracted: String,
}

#[allow(unused)]
impl Recipe {
    pub fn new(
        id: Option<i64>,
        name: String,
        url: Option<String>,
        image_url: Option<String>,
        extracted: String,
        owner_id: String,
    ) -> Self {
        Recipe {
            id,
            name,
            url,
            image_url,
            extracted,
            owner_id,
        }
    }

    pub fn url(&self) -> &str {
        self.url.as_deref().unwrap_or("https://example.com")
    }

    pub fn image_url(&self) -> &str {
        self.image_url
            .as_deref()
            .unwrap_or("https://example.com/image.jpg")
    }

    pub fn extracted(&self) -> &str {
        &self.extracted
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
                "Pumpkin Soup".to_string(),
                None,
                None,
                "Boil pumpkin and onions".to_string(),
                "1".to_string(),
            ),
            Recipe::new(
                None,
                "Chocolate Cake".to_string(),
                None,
                None,
                "Mix flour, sugar, eggs, and chocolate".to_string(),
                "2".to_string(),
            ),
        ]
    }
}
