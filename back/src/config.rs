use std::net::SocketAddr;

pub struct Config {
    pub addr: SocketAddr,
    pub database_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            addr: SocketAddr::from(([127, 0, 0, 1], 8000)),
            database_path: "../cleaner/rules.db".to_string(),
        }
    }
}