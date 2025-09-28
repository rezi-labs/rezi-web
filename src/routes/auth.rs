use crate::oidc::{AuthState, OidcClient};
use actix_session::Session;
use actix_web::{HttpRequest, HttpResponse, Result, get, web};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

type OidcClientArc = Arc<Mutex<OidcClient>>;

#[get("/auth/login")]
pub async fn login(
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

    let client = oidc_client.lock().unwrap();
    let auth_url = client
        .build_auth_url(&state, &code_challenge, auth_state.redirect_url.clone())
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Failed to build auth URL: {}", e))
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

    let client = oidc_client.lock().unwrap();

    let token_response = client
        .exchange_code(code, &auth_state.code_verifier)
        .await
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Token exchange failed: {}", e))
        })?;

    let user_info_result = client
        .get_user_info(&token_response.access_token)
        .await
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Failed to get user info: {}", e))
        })?;

    session.insert("user", &user_info_result)?;
    session.remove("auth_state");

    let redirect_url = auth_state.redirect_url.unwrap_or_else(|| "/".to_string());

    Ok(HttpResponse::Found()
        .append_header(("Location", redirect_url))
        .finish())
}

#[get("/auth/logout")]
pub async fn logout(session: Session) -> Result<HttpResponse> {
    session.purge();

    Ok(HttpResponse::Found()
        .append_header(("Location", "/"))
        .finish())
}
