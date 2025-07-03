use actix_web::HttpMessage;
use actix_web::Result;
use actix_web::web::Data;
use actix_web::{
    Error,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
};

use crate::config::Server;
use crate::unsafe_token_decode;

pub async fn user_extractor(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let config = req.app_data::<Data<Server>>().unwrap();

    if config.check_access_token() {
        let Some(at) = req.headers().get("X-Forwarded-Access-Token") else {
            return Err(actix_web::error::ErrorBadRequest("no token found"));
        };
        let Ok(header) = at.to_str() else {
            log::error!("could not get token from header");
            return Err(actix_web::error::ErrorBadRequest("invalid token"));
        };

        let user = match get_user(header) {
            Ok(user) => user,
            Err(e) => {
                log::error!("could not get user from token: {e}");
                return Err(actix_web::error::ErrorBadRequest("invalid token"));
            }
        };

        // insert user into app data
        req.extensions_mut().insert(Data::new(user));
    } else {
        if let Some(at) = req.headers().get("X-Forwarded-Access-Token") {
            if let Ok(header) = at.to_str() {
                match get_user(header) {
                    Ok(user) => log::info!("got user: {user:?}"),
                    Err(e) => {
                        log::error!("could not get user from token: {e}");
                    }
                };
            } else {
                log::error!("could not get token from header 50");
            };
        } else {
            log::error!("could not get token from header 60");
        };

        let user = unsafe_token_decode::User::new("0".to_string(), "guest@gmx.com".to_string());
        req.extensions_mut().insert(Data::new(user));
    }

    next.call(req).await
    // post-processing
}

pub fn get_user(token: &str) -> Result<unsafe_token_decode::User, String> {
    let user = unsafe_token_decode::decode_jwt_unsafe(token);

    let user = match user {
        Ok(user) => user,
        Err(e) => {
            log::error!("Invalid token: {e}");
            return Err("Invalid token".to_string());
        }
    };

    Ok(user)
}
