use rumors::ProviderType;
use std::env;

#[derive(Clone)]
pub struct Server {
    port: u16,
    host: String,
    db_url: String,
    token: Option<String>,
    provider_type: ProviderType,
    fake_user: bool,
    local: bool,
}

impl Server {
    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn host(&self) -> String {
        self.host.clone()
    }

    pub fn db_url(&self) -> String {
        self.db_url.clone()
    }
    pub fn db_token(&self) -> Option<String> {
        self.token.clone()
    }

    pub fn provider_type(&self) -> ProviderType {
        self.provider_type.clone()
    }

    pub fn delay(&self) -> bool {
        self.db_token().is_none()
    }

    pub fn local(&self) -> bool {
        self.local
    }

    pub fn fake_user(&self) -> bool {
        self.fake_user
    }
}

pub fn from_env() -> Server {
    let fake_user = env::var("FAKE_USER").unwrap_or("false".to_string());
    let fake_user = fake_user == "true";

    let local = env::var("LOCAL").unwrap_or("false".to_string());
    let local = local == "true";

    let port: u16 = env::var("g_port")
        .map(|e| e.parse().expect("could not parse port"))
        .unwrap_or(9999);
    let host: String = env::var("g_host")
        .map(|e| e.parse().expect("could not parse host"))
        .unwrap_or("0.0.0.0".to_string());

    let db_url: String = env::var("g_db_url")
        .map(|e| e.parse().expect("could not parse db url"))
        .unwrap_or("http://127.0.0.1:8080".to_string());
    let db_token: Option<String> = env::var("g_db_token").ok();

    // Determine provider type from environment
    let provider_type = if let Ok(api_key) = env::var("GOOGLE_API_KEY") {
        ProviderType::GoogleGemini { api_key }
    } else if let Ok(base_url) = env::var("OLLAMA_BASE_URL") {
        let model = env::var("OLLAMA_MODEL").unwrap_or_else(|_| "llama3.2".to_string());
        ProviderType::Ollama { base_url, model }
    } else {
        panic!("No AI provider configured. Set GOOGLE_API_KEY or OLLAMA_BASE_URL+OLLAMA_MODEL");
    };

    Server {
        port,
        host,
        db_url,
        token: db_token,
        provider_type,
        fake_user,
        local,
    }
}
