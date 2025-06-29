use std::env;

#[derive(Clone)]
pub struct Server {
    port: u16,
    host: String,
    db_url: String,
    token: Option<String>,
    nest_api: String,
    nest_api_key: String,
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

    pub fn nest_api(&self) -> String {
        self.nest_api.clone()
    }

    pub fn nest_api_key(&self) -> String {
        self.nest_api_key.clone()
    }

    pub fn delay(&self) -> bool {
        self.db_token().is_none()
    }
}

pub fn from_env() -> Server {
    let nest_api: String = env::var("NEST_API").unwrap_or("http://0.0.0.0:9998".to_string());
    let nest_api_key: String = env::var("NEST_API_KEY").expect("need NEST_API_KEY");

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
        nest_api,
        nest_api_key,
    }
}
