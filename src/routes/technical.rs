use actix_web::{HttpResponse, Responder, get};

#[get("/healthz")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok()
}
