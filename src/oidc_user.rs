use actix_session::SessionExt;
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
use crate::oidc;

pub async fn user_extractor(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let config = req.app_data::<Data<Server>>().unwrap();

    let session = req.get_session();

    let user = if config.get_user_from_headers() {
        match from_headers::get_user_from_headers(&req) {
            Ok(user) => user,
            Err(e) => {
                log::error!("could not get user from headers: {e}");
                return Err(actix_web::error::ErrorBadRequest(
                    "missing required headers",
                ));
            }
        }
    } else if let Some(oidc_user) = oidc::get_user_from_session(&session) {
        from_headers::User::new(oidc_user.sub, oidc_user.email)
    } else {
        from_headers::User::new("0".to_string(), "guest@gmx.com".to_string())
    };

    req.extensions_mut().insert(Data::new(user));
    next.call(req).await
}
