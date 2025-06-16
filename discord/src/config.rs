use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub token: String,
    pub application_id: String,
    pub gateway_url: String,
}

impl Config {
    /// Loads configuration from environment variables
    pub fn from_env() -> crate::Result<Self> {
        Ok(Self {
            token: env::var("DISCORD_TOKEN")?,
            application_id: env::var("DISCORD_APPLICATION_ID")?,
            gateway_url: env::var("DISCORD_GATEWAY_URL")
                .unwrap_or_else(|_| "wss://gateway.discord.gg/?v=10&encoding=json".to_string()),
        })
    }
}