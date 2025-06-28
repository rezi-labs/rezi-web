use actix_web::{App, HttpServer, middleware::Logger, web};
use env_logger::Env;
use std::sync::{Arc, Mutex};

use crate::{routes::health, view::todolist};

mod assets;
mod config;
mod database;
mod llm;
mod routes;
mod view;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    let c = config::from_env();
    let bind = c.clone();

    let db = database::create_client(c.db_url(), c.db_token()).await;

    let shared_db = Arc::new(Mutex::new(db));
    database::migrations(&shared_db).await;

    let url = format!("http://{}:{}", c.host(), c.port());

    let server = if webbrowser::open(&url).is_ok() {
        HttpServer::new(move || {
            App::new()
                .wrap(Logger::default())
                .wrap(Logger::new("%a %{User-Agent}i"))
                .app_data(web::Data::new(shared_db.clone()))
                .app_data(web::Data::new(c.clone()))
                .service(view::index_route)
                .service(todolist::index_route)
                .service(routes::send_message)
                .service(routes::create_item)
                .service(routes::toggle_item)
                .service(routes::delete_item)
                .service(assets::scope())
                .service(health)
        })
    } else {
        panic!("could not start")
    };

    server
        .bind((bind.host(), bind.port()))
        .expect("Could not bind server address")
        .run()
        .await
}
