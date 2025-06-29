use actix_web::HttpMessage;
use actix_web::Result;
use actix_web::web::Data;
use actix_web::{
    Error,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
};
use jsonwebtoken::TokenData;
use serde::Deserialize;

use crate::config::Server;

pub async fn user_extractor(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let config = req.app_data::<Data<Server>>().unwrap();

    if config.check_access_token() {
        let Some(at) = req.headers().get("X-Forwarded-Access-Token") else {
            return Err(actix_web::error::ErrorBadRequest("no token found"));
        };

        let user = get_user(at.to_str().unwrap()).unwrap();
        // insert user into app data
        req.extensions_mut().insert(Data::new(user));
    } else {
        let user = User::new("0".to_string(), "guest@gmx.com".to_string());
        req.extensions_mut().insert(Data::new(user));
    }

    next.call(req).await
    // post-processing
}

#[derive(Deserialize, Clone)]
pub struct User {
    id: String,
    email: String,
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

pub fn get_user(token: &str) -> Result<User, String> {
    let mut validation = jsonwebtoken::Validation::default();
    validation.insecure_disable_signature_validation();
    let key = jsonwebtoken::DecodingKey::from_secret(b"nothing");
    let user: jsonwebtoken::errors::Result<TokenData<User>> =
        jsonwebtoken::decode(token, &key, &validation);

    let user = match user {
        Ok(user) => user,
        Err(e) => {
            log::error!("Invalid token: {}", e);
            return Err("Invalid token".to_string());
        }
    };

    let user = user.claims;

    Ok(user)
}
