use actix_web::{HttpResponse, Responder, get, web};

use crate::ReloadArc;

#[get("/healthz")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/reload")]
pub async fn should_reload(reload: web::Data<ReloadArc>) -> impl Responder {
    let mut reload = reload.lock().unwrap();

    let should_reload = reload.reload();

    reload.set(false);

    if should_reload {
        HttpResponse::Ok()
            .append_header(("HX-Trigger", "reload"))
            .finish()
    } else {
        HttpResponse::Ok().finish()
    }
}
