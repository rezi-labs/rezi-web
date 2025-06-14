use actix_web::{get, post, HttpResponse, Responder};




#[get("/healthz")]
async fn health() -> impl Responder {
    HttpResponse::Ok()
}
