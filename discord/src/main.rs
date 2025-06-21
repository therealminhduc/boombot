mod error;
mod config;
mod gateway;
mod http;

use crate::error::Result;
use dotenv::dotenv;
use tracing::{info, error, warn};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(true)
        .with_max_level(tracing::Level::INFO)
        .init();

    dotenv().ok();
    info!("=== Discord bot starting ===");

    let config = config::Config::from_env()?;
    let http = http::DiscordClient::new(config.token.clone(), config.application_id.clone());

    // Register command once at startup 
    if let Err(e) = http.register_command().await { 
        error!("Failed to register command: {}", e);
        return Err(e);
    }
    info!("Registered /clean command");

    let mut reconnection_delay = Duration::from_secs(1);
    let max_delay = Duration::from_secs(300); // 5 minutes

    loop {
        match run_bot(&config, &http).await {
            Ok(_) => {
                info!("Bot is running ...");
                break;
            }
            Err(e) => {
                error!("Bot error: {}. Reconnecting in {:?} ...", e, reconnection_delay);
                sleep(reconnection_delay).await;

                // Exponential backoff: double the delay, but cap at max_delay
                reconnection_delay = std::cmp::min(reconnection_delay * 2, max_delay);
            }
        }
    }

    Ok(())
}

async fn run_bot(config: &config::Config, http: &http::DiscordClient) -> Result<()> {
    let mut gateway = gateway::Gateway::connect(&config.gateway_url, config.token.clone()).await?;
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
            info!("URL cleaned successfully, sending response");
            let response = format!("🧹 Cleaned URL:\n{}", cleaned);

            // @mynkie:
            // spawn: Create a new asynchronous task
            // async move: Mark the closure as async and takes ownership of the variables it uses 
            // (without taking ownership, the variables would be borrowed, which isn't allowed in async closures)
            
            tokio::spawn(async move {
                let start = std::time::Instant::now();
                let result = http.respond_to_interaction(&interaction_id, &token, &response).await;
                let elapsed = start.elapsed().as_millis();
                info!("Responded to interaction in {} ms", elapsed);
                if let Err(e) = result {
                    error!("Failed to respond to interaction: {}", e);
                } else {
                    info!("Response sent successfully");
                }
            });
        }
        Err(e) => {
            error!("Failed to clean URL: {}", e);
            let response = format!("❌ Error cleaning URL: {}", e);
            tokio::spawn(async move {
                let start = std::time::Instant::now();
                let result = http.respond_to_interaction(&interaction_id, &token, &response).await;
                let elapsed = start.elapsed().as_millis();
                info!("Responded to interaction (error) in {} ms", elapsed);
                if let Err(e) = result {
                    error!("Failed to respond to interaction: {}", e);
                }
            });
            return Err(error::BotError::CleaningError(e.to_string()));
        }
    }

    Ok(())
}

