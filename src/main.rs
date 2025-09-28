use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::cookie::Key;
use actix_web::{App, HttpServer, middleware::Logger, middleware::from_fn, web};
use env_logger::Env;
use std::sync::{Arc, Mutex};

use crate::{
    database::DBClient,
    oidc::{OidcClient, OidcConfig},
    view::items,
};

mod config;
mod csv;
mod database;
mod from_headers;
mod llm;
mod oidc;
mod oidc_user;
mod routes;
mod scrapy;
mod view;
mod witch;

pub struct Reload(bool);

impl Reload {
    pub fn new() -> Self {
        Reload(true)
    }

    pub fn reload(&self) -> bool {
        self.0
    }

    pub fn set(&mut self, value: bool) {
        self.0 = value;
    }
}

impl Default for Reload {
    fn default() -> Self {
        Reload::new()
    }
}

pub type ReloadArc = Arc<Mutex<Reload>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    let c = config::from_env();
    let bind = c.clone();

    let orm_db = database::create_orm_client(c.db_url(), c.db_token()).await;

    let shared_orm_db: DBClient = Arc::new(Mutex::new(orm_db));
    database::migrations::run(&shared_orm_db).await;

    let reload: ReloadArc = Arc::new(Mutex::new(Reload::default()));

    let oidc_config = OidcConfig::from_env();
    let mut oidc_client = OidcClient::new(oidc_config);

    if let Err(e) = oidc_client.discover().await {
        log::warn!("Failed to discover OIDC endpoints: {e}. OIDC authentication will be disabled.",);
    }

    let oidc_client_arc = Arc::new(tokio::sync::Mutex::new(oidc_client));

    let secret_key = std::env::var("SESSION_SECRET")
        .unwrap_or_else(|_| "your-secret-key-change-this-in-production".to_string());
    let secret_key = Key::from(secret_key.as_bytes());

    let url = format!("http://{}:{}", c.host(), c.port());

    println!("{}", rezi_asci_art());

    log::info!("Server started at {url}");

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default().exclude("/reload"))
            .wrap(Logger::new("%a %{User-Agent}i").exclude("/reload"))
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_name("rezi_session".to_string())
                    .cookie_secure(false) // Set to true in production with HTTPS
                    .cookie_http_only(true)
                    .build(),
            )
            .app_data(web::Data::new(shared_orm_db.clone()))
            .app_data(web::Data::new(c.clone()))
            .app_data(web::Data::new(reload.clone()))
            .app_data(web::Data::new(oidc_client_arc.clone()))
            // .wrap(from_fn(auth_middleware::require_auth_middleware)) // Disabled for now
            .wrap(from_fn(oidc_user::user_extractor))
            .service(routes::auth::login_page)
            .service(routes::auth::auth_login)
            .service(routes::auth::callback)
            .service(routes::auth::logout)
            .service(view::index_route)
            .service(view::chat_endpoint)
            .service(view::profile::profile_endpoint)
            .service(routes::recipes::recipe_endpoint)
            .service(routes::recipes::create_recipe)
            .service(routes::recipes::get_recipe)
            .service(routes::recipes::update_recipe)
            .service(routes::recipes::delete_recipe)
            .service(routes::recipes::edit_recipe)
            .service(routes::recipes::cancel_edit_recipe)
            .service(items::index_route)
            .service(routes::messages::send_message)
            .service(routes::messages::set_reply)
            .service(routes::messages::clear_reply)
            .service(routes::items::create_item_with_ai)
            .service(routes::items::create_item)
            .service(routes::items::toggle_item)
            .service(routes::items::delete_item)
            .service(routes::items::update_item)
            .service(routes::items::edit_item)
            .service(routes::items::cancel_edit_item)
            .service(routes::items::items_csv)
            .service(routes::technical::health)
            .service(routes::technical::should_reload)
            .service(routes::assets::scope())
    });
    server
        .bind((bind.host(), bind.port()))
        .expect("Could not bind server address")
        .run()
        .await
}

fn rezi_asci_art() -> &'static str {
    include_str!("../assets/rezi.txt")
}
