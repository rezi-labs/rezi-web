use actix_web::http::header::{CACHE_CONTROL, CONTENT_DISPOSITION};
use actix_web::{HttpRequest, HttpResponse, Result, get, web};

use crate::database::{self, DBClient};
use crate::{csv, pdf, view};

#[get("/export")]
pub async fn export_page(req: HttpRequest) -> Result<HttpResponse> {
    let user = match super::get_user_or_redirect(&req) {
        Ok(user) => user,
        Err(response) => return Ok(response),
    };

    let markup = view::export::export_page(&user);

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(markup.into_string()))
}

#[get("/export/items/csv")]
pub async fn export_items_csv(
    client: web::Data<DBClient>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let user = match super::get_user_or_redirect(&req) {
        Ok(user) => user,
        Err(response) => return Ok(response),
    };
    let owner_id = user.id().to_string();
    let db_client: &DBClient = client.get_ref();
    let items = database::items::get_items(db_client, owner_id)
        .await
        .unwrap_or_default();
    let csv_file = csv::items_to_events(items.as_slice());

    let response = HttpResponse::Ok()
        .append_header((CONTENT_DISPOSITION, "attachment; filename=\"items.csv\""))
        .append_header((CACHE_CONTROL, "no-cache"))
        .content_type("text/csv; charset=utf-8")
        .body(csv_file);

    Ok(response)
}

#[get("/export/items/pdf")]
pub async fn export_items_pdf(
    client: web::Data<DBClient>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let user = match super::get_user_or_redirect(&req) {
        Ok(user) => user,
        Err(response) => return Ok(response),
    };
    let owner_id = user.id().to_string();
    let db_client: &DBClient = client.get_ref();
    let items = database::items::get_items(db_client, owner_id)
        .await
        .unwrap_or_default();

    match pdf::items_to_pdf(items.as_slice()) {
        Ok(pdf_bytes) => {
            let response = HttpResponse::Ok()
                .append_header((CONTENT_DISPOSITION, "attachment; filename=\"items.pdf\""))
                .append_header((CACHE_CONTROL, "no-cache"))
                .content_type("application/pdf")
                .body(pdf_bytes);
            Ok(response)
        }
        Err(e) => {
            log::error!("Failed to generate PDF: {e}");
            Ok(HttpResponse::InternalServerError()
                .content_type("text/plain")
                .body("Failed to generate PDF"))
        }
    }
}
