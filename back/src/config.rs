use std::net::{SocketAddr, IpAddr};
use std::env;
use std::str::FromStr;

pub struct Config {
    pub addr: SocketAddr,
    pub database_path: String,
}

impl Default for Config {
    fn default() -> Self {
        // Get host and port from environment variables or use defaults
        let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(8000);

        // Parse host into IP address
        let ip = IpAddr::from_str(&host).unwrap_or(IpAddr::from([127, 0, 0, 1]));
        
        Self {
            addr: SocketAddr::from((ip, port)),
            database_path: env::var("DATABASE_PATH")
                .unwrap_or_else(|_| "../cleaner/rules.db".to_string()),
        }
    }
}