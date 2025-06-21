use crate::error::Result;
use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use tokio_tungstenite::{connect_async, tungstenite::Message, WebSocketStream};
use url::Url;
use tracing::{error, info, warn};

pub struct Gateway {
    ws_stream: WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
    token: String,
}

impl Gateway {

    /// Connects to the Discord Gateway WebSocket endpoint. 
    /// Returns a Gateway struct that contains the WebSocket stream and the bot token.
    pub async fn connect(gateway_url: &str, token: String) -> Result<Self> {
        info!("Connecting to Discord Gateway: {}", gateway_url);
        
        let (ws_stream, _) = connect_async(Url::parse(gateway_url)?).await?;

        Ok(Self {
            ws_stream,
            token,
        })
    }

    /// Sends the "Identify" payload to Discord
    /// This tells Discord who we are and what we want to do
    pub async fn identify(&mut self) -> Result<()> {
        info!("Identifying to Discord Gateway");
        
        let identify = json!({
            "op": 2,    // 2 = Identify - Starts a new session during the initial handshake.
            "d": {
                "token": self.token,
                "intents": 513, // Intents: 513 = Guilds, GuildMessages, MessageContent
                "properties": {
                    "$os": std::env::consts::OS,
                    "$browser": "boombot",
                    "$device": "boombot",
                }
            }
        });

        // Send the identify payload to the Discord Gateway
        self.ws_stream
            .send(Message::Text(identify.to_string()))
            .await?;

        Ok(())
    }

    /// Handle events from Discord Gateway using a callback function that processes each event as it arrives.
    pub async fn handle_events<F>(&mut self, mut callback: F) -> Result<()>
    where
        F: FnMut(serde_json::Value) -> Result<()>,
    {
        // Listen for events from the Discord Gateway
        while let Some(message) = self.ws_stream.next().await {
            match message {
                Ok(msg) => {
                    match msg {
                        Message::Text(text) => {
                            // Discord sent us a text message (JSON)
                            match serde_json::from_str::<serde_json::Value>(&text) {
                                Ok(value) => {
                                    // Check if this is a Discord protocol message (has an "op" field)
                                    if let Some(op) = value.get("op").and_then(|v| v.as_u64()) {
                                        match op {
                                            1 => {
                                                // Discord is requesting a heartbeat (aka Discord: "Are you alive ?"), we respond back with a heartbeat
                                                info!("Received heartbeat request from Discord");
                                                let heartbeat = json!({
                                                    "op": 1,
                                                    "d": null,
                                                });

                                                // Send the heartbeat to Discord (aka Bot: "I'm alive !")
                                                // If the heartbeat is not sent, Discord will consider the bot as dead and will close the connection
                                                if let Err(e) = self.ws_stream.send(Message::Text(heartbeat.to_string())).await {
                                                    error!("Failed to send heartbeat: {}", e);
                                                    break;
                                                }
    
                                                continue;
                                            }

                                            10 => {
                                                // Op code 10 = Hello message
                                                // Discord is telling us the heartbeat interval
                                                if let Some(d) = value.get("d") {
                                                    if let Some(heartbeat_interval) = d.get("heartbeat_interval").and_then(|v| v.as_u64()) {
                                                        info!("Heartbeat interval: {}ms", heartbeat_interval);
                                                    }
                                                }
                                            }

                                            _ => {
                                                // Ignore other op codes
                                            }
                                        }
                                    }

                                    // Pass the message to our callback function (handles events like INTERACTION_CREATE)
                                    if let Err(e) = callback(value) {
                                        error!("Error processing event: {}", e);
                                        // Continue processing the next message instead of breaking
                                    }
                                }

                                Err(e) => {
                                    error!("Failed to parse message as JSON: {}", e);
                                }
                            }
                        }

                        Message::Close(frame) => {
                            if let Some(frame) = frame {
                                warn!("WebSocket connection closed: {} - {}", frame.code, frame.reason);
                            } else {
                                warn!("WebSocket connection closed without a close frame");
                            }

                            break;
                        }

                        Message::Ping(data) => {
                            // Discord sent a WebSocket ping (connection health check)
                            // We must respond with a pong to keep the connection alive
                            if let Err(e) = self.ws_stream.send(Message::Pong(data)).await {
                                error!("Failed to send pong: {}", e);
                                break;
                            }
                        }

                        _ => {
                            // Ignore other message types
                        }
                    }
                }

                Err(e) => {
                    error!("Error reading message from Discord: {}", e);
                    break;
                }
            }
        }

        Ok(())
    }
}
