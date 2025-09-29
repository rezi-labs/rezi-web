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
use log::debug;
use log::info;
use log::warn;

use crate::config::Server;
use crate::oidc;
use crate::user;
use crate::user::User;

pub async fn user_extractor(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let config = req.app_data::<Data<Server>>().unwrap();

    let session = req.get_session();

    let user: Option<User> = if config.fake_user() {
        let u = user::User::new("0".to_string(), "guest@gmx.com".to_string());
        warn!("Using fake user");
        Some(u)
    } else if let Some(oidc_user) = oidc::get_user_from_session(&session) {
        let u = user::User::new(oidc_user.sub, oidc_user.email);
        debug!("Using OIDC user");
        Some(u)
    } else {
        info!("No user found, anonymous");
        None
    };

    if let Some(user) = user {
        req.extensions_mut().insert(Data::new(user));
    }
    next.call(req).await
}
