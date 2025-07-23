use actix_web::{App, HttpServer, middleware::Logger, middleware::from_fn, web};
use env_logger::Env;
use std::{
    env,
    sync::{Arc, Mutex},
};

use crate::{routes::health, view::items};

mod assets;
mod config;
mod csv;
mod database;
mod ical;
mod llm;
mod routes;
mod scrapy;
mod unsafe_token_decode;
mod user;
mod view;
mod witch;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    let c = config::from_env();
    let bind = c.clone();

    let db = database::create_client(c.db_url(), c.db_token()).await;

    let shared_db = Arc::new(Mutex::new(db));
    database::migrations::run(&shared_db, &c).await;

    let url = format!("http://{}:{}", c.host(), c.port());

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(web::Data::new(shared_db.clone()))
            .app_data(web::Data::new(c.clone()))
            .wrap(from_fn(user::user_extractor))
            .service(view::index_route)
            .service(view::chat_endpoint)
            .service(view::profile::profile_endpoint)
            .service(view::recipes::recipe_endpoint)
            .service(view::info::info_endpoint)
            .service(items::index_route)
            .service(routes::send_message)
            .service(routes::create_item_with_ai)
            .service(routes::create_item)
            .service(routes::toggle_item)
            .service(routes::delete_item)
            .service(routes::update_item)
            .service(routes::edit_item)
            .service(routes::cancel_edit_item)
            .service(routes::items_ical)
            .service(routes::items_csv)
            .service(assets::scope())
            .service(health)
    });

    if env::var("OPEN_BROWSER").is_ok() {
        webbrowser::open(&url).unwrap();
    }

    server
        .bind((bind.host(), bind.port()))
        .expect("Could not bind server address")
        .run()
        .await
}
