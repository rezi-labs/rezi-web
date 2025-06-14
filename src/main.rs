use actix_web::{App, HttpServer, middleware::Logger, web};
use env_logger::Env;

use crate::assets::scope;

mod assets;
mod config;
mod llm;
mod routes;
mod view;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // Initialize sample todo data
    routes::initialize_sample_data();

    let c = config::from_env();

    let url = format!("http://{}:{}/ui", c.host(), c.port());

    let server = if webbrowser::open(&url).is_ok() {
        HttpServer::new(|| {
            App::new()
                .wrap(Logger::default())
                .wrap(Logger::new("%a %{User-Agent}i"))
                .service(view::scope())
                .service(
                    web::scope("/api")
                        .service(routes::health)
                        .service(routes::todo_scope())
                        .service(routes::chat_scope()),
                )
                .service(assets::scope())
        })
    } else {
        panic!("could not start")
    };

    let res = server
        .bind((c.host(), c.port()))
        .expect("Could not bind server address")
        .run()
        .await;

    res
}
