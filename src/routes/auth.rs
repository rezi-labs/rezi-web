use crate::oidc::{AuthState, OidcClient};
use crate::view::login;
use actix_session::Session;
use actix_web::{HttpRequest, HttpResponse, Result, get, web};
use log::{error, info};
use maud::Markup;
use std::sync::Arc;
use uuid::Uuid;

type OidcClientArc = Arc<tokio::sync::Mutex<OidcClient>>;

#[get("/login")]
pub async fn login_page() -> Result<Markup> {
    Ok(login::login_page())
}

#[get("/auth/login")]
pub async fn auth_login(
    req: HttpRequest,
    session: Session,
    oidc_client: web::Data<OidcClientArc>,
) -> Result<HttpResponse> {
    let redirect_to = req
        .query_string()
        .split('=')
        .nth(1)
        .map(|s| urlencoding::decode(s).unwrap_or_default().to_string());

    let state = Uuid::new_v4().to_string();
    let (code_verifier, code_challenge) = crate::oidc::OidcClient::generate_pkce();

    let auth_state = AuthState {
        state: state.clone(),
        code_verifier,
        redirect_url: redirect_to,
    };

    session.insert("auth_state", &auth_state)?;

    let client = oidc_client.lock().await;
    let auth_url = client
        .build_auth_url(&state, &code_challenge, auth_state.redirect_url.clone())
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Failed to build auth URL: {e}"))
        })?;

    Ok(HttpResponse::Found()
        .append_header(("Location", auth_url))
        .finish())
}

#[get("/auth/callback")]
pub async fn callback(
    req: HttpRequest,
    session: Session,
    oidc_client: web::Data<OidcClientArc>,
) -> Result<HttpResponse> {
    info!(
        "Callback - Session entries at start: {:?}",
        session.entries()
    );
    info!("Callback - Session status: {:?}", session.status());

    let query = req.query_string();
    let params: std::collections::HashMap<String, String> =
        url::form_urlencoded::parse(query.as_bytes())
            .into_owned()
            .collect();

    let code = params
        .get("code")
        .ok_or_else(|| actix_web::error::ErrorBadRequest("Missing authorization code"))?;

    let state = params
        .get("state")
        .ok_or_else(|| actix_web::error::ErrorBadRequest("Missing state parameter"))?;

    let auth_state: AuthState = session
        .get("auth_state")?
        .ok_or_else(|| actix_web::error::ErrorBadRequest("Missing auth state in session"))?;

    if auth_state.state != *state {
        return Err(actix_web::error::ErrorBadRequest("Invalid state parameter"));
    }

    let token_response = {
        let client = oidc_client.lock().await;
        client
            .exchange_code(code, &auth_state.code_verifier)
            .await
            .map_err(|e| {
                actix_web::error::ErrorInternalServerError(format!("Token exchange failed: {e}"))
            })?
    };

    let user_info_result = {
        let client = oidc_client.lock().await;
        client
            .get_user_info(&token_response.access_token)
            .await
            .map_err(|e| {
                actix_web::error::ErrorInternalServerError(format!("Failed to get user info: {e}"))
            })?
    };

    // Still try the original struct approach
    match session.insert("user", &user_info_result) {
        Ok(_) => info!("Successfully stored user struct in session"),
        Err(e) => error!("Failed to store user struct in session: {e}"),
    }

    info!(
        "Final session entries before removing auth_state: {:?}",
        session.entries()
    );

    session.remove("auth_state");

    let redirect_url = auth_state.redirect_url.unwrap_or_else(|| "/".to_string());

    // Make sure session is committed before redirect
    info!("About to redirect to: {redirect_url}");

    // Instead of redirect, let's try returning HTML with meta refresh to ensure session is saved
    let html_response = format!(
        r#"
    <!DOCTYPE html>
    <html>
    <head>
        <meta http-equiv="refresh" content="1;url={redirect_url}">
        <title>Redirecting...</title>
    </head>
    <body>
        <p>Login successful. Redirecting in 1 second...</p>
        <p>If you are not redirected automatically, <a href="{redirect_url}">click here</a>.</p>
    </body>
    </html>
    "#
    );

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(html_response))
}

#[get("/auth/logout")]
pub async fn logout(session: Session) -> Result<HttpResponse> {
    session.purge();

    Ok(HttpResponse::Found()
        .append_header(("Location", "/"))
        .finish())
}
