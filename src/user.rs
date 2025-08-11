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
use crate::from_headers;

pub async fn user_extractor(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let config = req.app_data::<Data<Server>>().unwrap();

    if config.get_user_from_headers() {
        // Extract user from headers instead of token
        let user = match from_headers::get_user_from_headers(&req) {
            Ok(user) => user,
            Err(e) => {
                log::error!("could not get user from headers: {e}");
                return Err(actix_web::error::ErrorBadRequest(
                    "missing required headers",
                ));
            }
        };

        // insert user into app data
        req.extensions_mut().insert(Data::new(user));
    } else {
        // Try to get user from headers in dev mode
        if let Ok(user) = from_headers::get_user_from_headers(&req) {
            log::info!("got user from headers: {user:?}");
            req.extensions_mut().insert(Data::new(user));
        } else {
            log::info!("no user headers found, using guest user");
            let user = from_headers::User::new("0".to_string(), "guest@gmx.com".to_string());
            req.extensions_mut().insert(Data::new(user));
        }
    }

    next.call(req).await
    // post-processing
}
