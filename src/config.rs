use std::env;

pub struct Server {
    port: u16,
    host: String,
    db_url: String,
}

impl Server {
    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn host(&self) -> String {
        self.host.clone()
    }
    
    pub fn db_url(&self) -> String{
        self.db_url.clone()
    }
}

pub fn from_env() -> Server {
    let port: u16 = env::var("g_port")
        .map(|e| e.parse().expect("could not parse port"))
        .unwrap_or(9999);
    let host: String = env::var("g_host")
        .map(|e| e.parse().expect("could not parse host"))
        .unwrap_or("0.0.0.0".to_string());

    let db_url: String = env::var("g_db_url")
        .map(|e| e.parse().expect("could not parse db url"))
        .unwrap_or("http://127.0.0.1:8080".to_string());

    Server { port, host, db_url }
}
