use crate::error::Result;
use reqwest::Client;

#[derive(Clone)]
pub struct DiscordClient {
    client: Client,
    token: String,
    application_id: String,
}

impl DiscordClient {
    pub fn new(token: String, application_id: String) -> Self {
        Self {
            client: Client::new(),
            token,
            application_id,
        }
    }

    /// Registers the /clean command
    /// This registration is permanent and will remain in the server's command list until it is removed.
    pub async fn register_command(&self) -> Result<()> {
            let url = format!(
                "https://discord.com/api/v10/applications/{application_id}/commands",
                application_id = self.application_id
            );

        let command = serde_json::json!({
            "name": "clean",
            "description": "Clean tracking parameters from a URL",
            "options": [{
                "name": "url",
                "description": "The URL to clean",
                "type": 3, // String type
                "required": true,
            }]
        });

        let response = self.client.post(&url)
            .header("Authorization", format!("Bot {token}", token = self.token))
            .json(&command)
            .send()
            .await?;

        if !response.status().is_success() {
            let error = response.text().await?;
            return Err(crate::error::BotError::InvalidPayload(error));
        }

        Ok(())
    }

    /// Responds to an interaction with a message.
    /// Used to send a response to the user who triggered the command.
    pub async fn respond_to_interaction(
        &self,
        interaction_id: &str,
        interaction_token: &str,
        content: &str,
    ) -> Result<()> {
        let url = format!(
            "https://discord.com/api/v10/interactions/{interaction_id}/{interaction_token}/callback",
        );

        let response = serde_json::json!({
            "type": 4,
            "data": {
                "content": content,
            }
        });

        self.client.post(&url)
            .header("Authorization", format!("Bot {token}", token = self.token))
            .json(&response)
            .send()
            .await?;

        Ok(())
    }
}