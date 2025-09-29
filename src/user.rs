use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct User {
    pub id: String,
    pub email: String,
}

impl User {
    pub fn new(id: String, email: String) -> Self {
        User { id, email }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn initials(&self) -> String {
        let mut split = self.email().split(".");
        let first_name = split.next().unwrap_or("Unknown");
        let last_name = split.next().unwrap_or("User");
        format!(
            "{}{}",
            first_name.chars().next().unwrap_or(' ').to_uppercase(),
            last_name.chars().next().unwrap_or(' ').to_uppercase()
        )
    }
}
