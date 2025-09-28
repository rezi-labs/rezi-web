use actix_web::{
    Error,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    HttpResponse,
};
use actix_session::SessionExt;

pub async fn require_auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let path = req.path();
    
    // Allow access to auth endpoints, assets, and health check
    if path.starts_with("/auth/") || path.starts_with("/assets/") || path == "/health" {
        return next.call(req).await;
    }

    let session = req.get_session();
    
    if !crate::oidc::is_authenticated(&session) {
        // For HTMX requests, redirect to login
        let is_htmx = req.headers().contains_key("hx-request");
        
        if is_htmx {
            let login_url = format!("/auth/login?redirect_to={}", urlencoding::encode(path));
            let response = HttpResponse::Found()
                .append_header(("HX-Redirect", login_url))
                .finish();
            let (http_req, _) = req.into_parts();
            return Ok(ServiceResponse::new(http_req, response).map_into_boxed_body());
        } else {
            // For regular requests, show login page
            let login_html = crate::view::index(
                Some(crate::view::login::login_page()), 
                false
            );
            let response = HttpResponse::Ok()
                .content_type("text/html")
                .body(login_html.into_string());
            let (http_req, _) = req.into_parts();
            return Ok(ServiceResponse::new(http_req, response).map_into_boxed_body());
        }
    }

    next.call(req).await
}