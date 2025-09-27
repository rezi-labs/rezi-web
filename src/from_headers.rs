use actix_web::dev::ServiceRequest;
use log::info;
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
            first_name.chars().next().unwrap_or(' '),
            last_name.chars().next().unwrap_or(' ')
        )
    }
}

pub fn get_user_from_headers(req: &ServiceRequest) -> Result<User, String> {
    let headers = req.headers();

    // Try X-Forwarded-User first (preferred username/ID)
    let user_id = headers
        .get("X-Forwarded-User")
        .or_else(|| headers.get("X-Forwarded-Preferred-Username"))
        .and_then(|h| h.to_str().ok())
        .unwrap_or("unknown")
        .to_string();

    // Get email from X-Forwarded-Email
    let email = headers
        .get("X-Forwarded-Email")
        .and_then(|h| h.to_str().ok())
        .ok_or("Missing X-Forwarded-Email header")?
        .to_string();

    info!("{headers:?}");

    Ok(User::new(user_id, email))
}
