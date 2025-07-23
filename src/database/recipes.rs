#[allow(unused)]
pub struct Recipe {
    id: String,
    owner_id: String,
    name: String,
    url: Option<String>,
    image_url: Option<String>,
    extracted: String,
}

#[allow(unused)]
impl Recipe {
    pub fn new(
        id: String,
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

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn examples() -> Vec<Recipe> {
        vec![
            Recipe::new(
                "".to_string(),
                "Pumpkin Soup".to_string(),
                None,
                None,
                "Boil pumpkin and onions".to_string(),
                "1".to_string(),
            ),
            Recipe::new(
                "".to_string(),
                "Chocolate Cake".to_string(),
                None,
                None,
                "Mix flour, sugar, eggs, and chocolate".to_string(),
                "2".to_string(),
            ),
        ]
    }
}
