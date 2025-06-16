mod error;
mod config;
mod gateway;
mod http;

use crate::error::Result;
use dotenv::dotenv;
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    dotenv().ok();
    info!("Starting Discord bot ...");

    let config = config::Config::from_env()?;

    let http = http::DiscordClient::new(config.token.clone(), config.application_id.clone());

    http.register_command().await?;
    info!("Registered /clean command");

    let mut gateway = gateway::Gateway::connect(&config.gateway_url, config.token).await?;
    gateway.identify().await?;
    info!("Connected to Discord Gateway");

    gateway.handle_events(|event| {
        // Check if the event has a type field (t = Event name)
        if let Some(t) = event.get("t").and_then(|v| v.as_str()) {
            match t {
                // When the bot is ready and connected to the Discord Gateway
                "READY" => {
                    info!("Bot is ready");
                }

                // When someone uses the /clean command
                "INTERACTION_CREATE" => {
                    if let Some(data) = event.get("d") {
                        handle_interaction(data, &http)?;
                    }
                }

                // Ignore other events
                _ => {}
            }
        }

        Ok(())
        
        })
        .await?;

    Ok(())
}

fn handle_interaction(data: &serde_json::Value, http: &http::DiscordClient) -> Result<()> {
    // Get the interaction ID and token from the event data
    let interaction_id = data["id"].as_str().ok_or_else(|| 
        error::BotError::InvalidPayload("Missing interaction ID".to_string())
    )?;

    let token = data["token"].as_str().ok_or_else(|| 
        error::BotError::InvalidPayload("Missing interaction token".to_string())
    )?;

    // Navigate through the JSON structure and get the command name from the event data.
    let command = data.get("data")
        .and_then(|d| d.get("name"))
        .and_then(|n| n.as_str())
        .ok_or_else(|| error::BotError::InvalidPayload("Missing command name".to_string()))?;

    if command != "clean" {
        return Ok(());
    }

    // Navigate through the JSON structure and get the URL from the event data.
    let url = data["data"]["options"][0]["value"]
        .as_str()
        .ok_or_else(|| error::BotError::InvalidPayload("Missing URL parameter".to_string()))?;

    let interaction_id = interaction_id.to_string();
    let token = token.to_string();
    let http = http.clone();

    // Clean the URL
    match cleaner::clean_url(url) {
        Ok(cleaned) => {
            let response = format!("🧹 Cleaned URL:\n{}", cleaned);

            // @mynkie:
            // spawn: Create a new asynchronous task
            // async move: Mark the closure as async and takes ownership of the variables it uses 
            // (without taking ownership, the variables would be borrowed, which isn't allowed in async closures)
            
            tokio::spawn(async move {
                if let Err(e) = http.respond_to_interaction(&interaction_id, &token, &response).await {
                    error!("Failed to respond to interaction: {}", e);
                }
            });
        }
        Err(e) => {
            let response = format!("❌ Error cleaning URL: {}", e);
            tokio::spawn(async move {
                if let Err(e) = http.respond_to_interaction(&interaction_id, &token, &response).await {
                    error!("Failed to respond to interaction: {}", e);
                }
            });
            return Err(error::BotError::CleaningError(e.to_string()));
        }
    }

    Ok(())
}

