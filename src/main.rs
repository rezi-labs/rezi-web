use actix_web::{App, HttpServer, middleware::Logger, middleware::from_fn, web};
use env_logger::Env;
use std::sync::{Arc, Mutex};

use crate::{database::DBClient, view::items};

mod config;
mod csv;
mod database;
mod llm;
mod routes;
mod scrapy;
mod unsafe_token_decode;
mod user;
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

    let url = format!("http://{}:{}", c.host(), c.port());

    println!("{}", rezi_asci_art());

    log::info!("Server started at {url}");

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(web::Data::new(shared_orm_db.clone()))
            .app_data(web::Data::new(c.clone()))
            .app_data(web::Data::new(reload.clone()))
            .wrap(from_fn(user::user_extractor))
            .service(view::index_route)
            .service(view::chat_endpoint)
            .service(view::profile::profile_endpoint)
            .service(view::recipes::recipe_endpoint)
            .service(view::info::info_endpoint)
            .service(items::index_route)
            .service(routes::messages::send_message)
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
