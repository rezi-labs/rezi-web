use actix_session::Session;
use actix_web::{HttpResponse, Result};
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct OidcConfig {
    pub client_id: String,
    pub client_secret: String,
    pub issuer_url: String,
    pub redirect_uri: String,
    pub scopes: String,
}

impl OidcConfig {
    pub fn from_env() -> Self {
        Self {
            client_id: std::env::var("OIDC_CLIENT_ID")
                .unwrap_or_else(|_| "default-client-id".to_string()),
            client_secret: std::env::var("OIDC_CLIENT_SECRET")
                .unwrap_or_else(|_| "default-client-secret".to_string()),
            issuer_url: std::env::var("OIDC_ISSUER_URL")
                .unwrap_or_else(|_| "https://accounts.google.com".to_string()),
            redirect_uri: std::env::var("OIDC_REDIRECT_URI")
                .unwrap_or_else(|_| "http://localhost:3000/auth/callback".to_string()),
            scopes: std::env::var("OIDC_SCOPES")
                .unwrap_or_else(|_| "openid email profile".to_string()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OidcDiscovery {
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    pub userinfo_endpoint: String,
    pub jwks_uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: Option<u64>,
    pub id_token: Option<String>,
    pub refresh_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserInfo {
    pub sub: String,
    pub email: String,
    pub name: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthState {
    pub state: String,
    pub code_verifier: String,
    pub redirect_url: Option<String>,
}

pub struct OidcClient {
    config: OidcConfig,
    client: reqwest::Client,
    discovery: Option<OidcDiscovery>,
}

impl OidcClient {
    pub fn new(config: OidcConfig) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
            discovery: None,
        }
    }

    pub async fn discover(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let discovery_url = format!(
            "{}/.well-known/openid_configuration",
            self.config.issuer_url
        );
        let discovery: OidcDiscovery = self.client.get(&discovery_url).send().await?.json().await?;

        self.discovery = Some(discovery);
        Ok(())
    }

    pub fn generate_pkce() -> (String, String) {
        let code_verifier = URL_SAFE_NO_PAD.encode(Uuid::new_v4().as_bytes());
        let mut hasher = Sha256::new();
        hasher.update(code_verifier.as_bytes());
        let code_challenge = URL_SAFE_NO_PAD.encode(hasher.finalize());
        (code_verifier, code_challenge)
    }

    pub fn build_auth_url(
        &self,
        state: &str,
        code_challenge: &str,
        redirect_url: Option<String>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let discovery = self
            .discovery
            .as_ref()
            .ok_or("OIDC discovery not completed")?;

        let final_redirect_uri = match redirect_url {
            Some(url) => format!(
                "{}?return_to={}",
                self.config.redirect_uri,
                urlencoding::encode(&url)
            ),
            None => self.config.redirect_uri.clone(),
        };

        let params = vec![
            ("response_type", "code"),
            ("client_id", &self.config.client_id),
            ("redirect_uri", &final_redirect_uri),
            ("scope", &self.config.scopes),
            ("state", state),
            ("code_challenge", code_challenge),
            ("code_challenge_method", "S256"),
        ];

        let query_string = params
            .iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        Ok(format!(
            "{}?{}",
            discovery.authorization_endpoint, query_string
        ))
    }

    pub async fn exchange_code(
        &self,
        code: &str,
        code_verifier: &str,
    ) -> Result<TokenResponse, Box<dyn std::error::Error>> {
        let discovery = self
            .discovery
            .as_ref()
            .ok_or("OIDC discovery not completed")?;

        let mut params = HashMap::new();
        params.insert("grant_type", "authorization_code");
        params.insert("client_id", &self.config.client_id);
        params.insert("client_secret", &self.config.client_secret);
        params.insert("code", code);
        params.insert("redirect_uri", &self.config.redirect_uri);
        params.insert("code_verifier", code_verifier);

        let response: TokenResponse = self
            .client
            .post(&discovery.token_endpoint)
            .form(&params)
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }

    pub async fn get_user_info(
        &self,
        access_token: &str,
    ) -> Result<UserInfo, Box<dyn std::error::Error>> {
        let discovery = self
            .discovery
            .as_ref()
            .ok_or("OIDC discovery not completed")?;

        let user_info: UserInfo = self
            .client
            .get(&discovery.userinfo_endpoint)
            .bearer_auth(access_token)
            .send()
            .await?
            .json()
            .await?;

        Ok(user_info)
    }
}

pub fn is_authenticated(session: &Session) -> bool {
    session.get::<UserInfo>("user").unwrap_or(None).is_some()
}

pub fn get_user_from_session(session: &Session) -> Option<UserInfo> {
    session.get::<UserInfo>("user").unwrap_or(None)
}

pub fn require_auth(session: &Session, current_path: &str) -> Result<Option<HttpResponse>> {
    if !is_authenticated(session) {
        let login_url = format!(
            "/auth/login?redirect_to={}",
            urlencoding::encode(current_path)
        );
        return Ok(Some(
            HttpResponse::Found()
                .append_header(("Location", login_url))
                .finish(),
        ));
    }
    Ok(None)
}
