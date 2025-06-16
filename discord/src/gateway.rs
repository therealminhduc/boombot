use crate::error::Result;
use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use tokio_tungstenite::connect_async;
use url::Url;

pub struct Gateway {
    ws_stream: tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
    token: String,
}

impl Gateway {

    /// Connects to the Discord Gateway WebSocket endpoint. 
    /// Returns a Gateway struct that contains the WebSocket stream and the bot token.
    pub async fn connect(gateway_url: &str, token: String) -> Result<Self> {
        let (ws_stream, _) = connect_async(Url::parse(gateway_url)?).await?;

        Ok(Self {
            ws_stream,
            token,
        })
    }

    /// Identify to Discord Gateway - Starts a new session during the initial handshake.
    pub async fn identify(&mut self) -> Result<()> {
        let identify = json!({
            "op": 2,    // 2 = Identify - Starts a new session during the initial handshake.
            "d": {
                "token": self.token,
                "intents": 513, // Intents: 513 = Guilds, GuildMessages, MessageContent
                "properties": {
                    "$os": std::env::consts::OS,
                    "$browser": "chrome",
                    "$device": "chrome",
                }
            }
        });

        // Send the identify payload to the Discord Gateway
        self.ws_stream
            .send(tokio_tungstenite::tungstenite::Message::Text(identify.to_string()))
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
            let message = message?;

            // If the message is a text message, parse it as a JSON value and call the callback function.
            if let tokio_tungstenite::tungstenite::Message::Text(text) = message {
                let value: serde_json::Value = serde_json::from_str(&text)?;
                callback(value)?;
            }
        }

        Ok(())
    }
}
