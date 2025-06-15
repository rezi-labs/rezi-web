use actix_web::{App, HttpServer, middleware::Logger, web};
use env_logger::Env;
use std::sync::{Arc, Mutex};

mod assets;
mod config;
mod database;
mod llm;
mod routes;
mod view;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    
    let c = config::from_env();

    let db = database::create_client(c.db_url()).await;

    let shared_db = Arc::new(Mutex::new(db));
    database::migrations(&shared_db).await;


    let url = format!("http://{}:{}/ui", c.host(), c.port());

    let server = if webbrowser::open(&url).is_ok() {
        HttpServer::new(move || {
            App::new()
                .wrap(Logger::default())
                .wrap(Logger::new("%a %{User-Agent}i"))
                .app_data(web::Data::new(shared_db.clone()))
                .service(view::scope())
                .service(
                    web::scope("/api")
                        .service(routes::health)
                        .service(routes::item_scope())
                        .service(routes::chat_scope()),
                )
                .service(assets::scope())
        })
    } else {
        panic!("could not start")
    };

    server
        .bind((c.host(), c.port()))
        .expect("Could not bind server address")
        .run()
        .await
}
