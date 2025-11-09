use std::env;

#[derive(Clone)]
pub struct Server {
    port: u16,
    host: String,
    db_url: String,
    token: Option<String>,

    llm_provider: String,
    llm_api_key: String,
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

    #[allow(dead_code)]
    pub fn delay(&self) -> bool {
        self.db_token().is_none()
    }

    pub fn local(&self) -> bool {
        self.local
    }

    pub fn fake_user(&self) -> bool {
        self.fake_user
    }

    pub fn llm_provider(&self) -> String {
        self.llm_provider.clone()
    }

    pub fn llm_api_key(&self) -> String {
        self.llm_api_key.clone()
    }
}

pub fn from_env() -> Server {
    let fake_user = env::var("FAKE_USER").unwrap_or("false".to_string());
    let fake_user = fake_user == "true";

    let local = env::var("LOCAL").unwrap_or("false".to_string());
    let local = local == "true";

    // LLM Configuration - defaults to using Gemini
    let llm_provider: String = env::var("LLM_PROVIDER").unwrap_or("gemini".to_string());
    let llm_api_key: String = env::var("LLM_API_KEY")
        .or_else(|_| env::var("GEMINI_API_KEY"))
        .or_else(|_| env::var("OPENAI_API_KEY"))
        .expect("Need LLM_API_KEY, GEMINI_API_KEY, or OPENAI_API_KEY");

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
    Server {
        port,
        host,
        db_url,
        token: db_token,

        llm_provider,
        llm_api_key,
        fake_user,
        local,
    }
}
