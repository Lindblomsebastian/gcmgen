use crate::client::CommitMessageGenerator;
use crate::config::ServiceConfig;
use reqwest::blocking::Client;
use serde_json::json;
use std::error::Error;

pub struct AnthropicClient {
    api_token: String,
    client: Client,
    model: String,
}

impl AnthropicClient {
    pub fn new(service_config: &ServiceConfig) -> Self {
        AnthropicClient {
            api_token: service_config.api_token.clone(),
            model: service_config.model.clone(),
            client: Client::new(),
        }
    }
}

impl CommitMessageGenerator for AnthropicClient {
    fn generate_commit_message(
        &self,
        diff: &str,
        prefix: Option<&String>,
    ) -> Result<String, Box<dyn Error>> {
        let messages = json!([
            {
                "role": "system",
                "content": "You are a helpful assistant specialized in writing concise and meaningful git commit messages."
            },
            {
                "role": "user",
                "content": format!("Here is a git diff:\n\n{}", diff)
            },
            {
                "role": "assistant",
                "content": "Generate a concise and meaningful commit message based on the provided git diff.\n\
                Only include the concise and meaningful commit message. Don't include any text formatting."
            }
        ]);

        let response = self
            .client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_token)
            .json(&json!({
                "model": &self.model,
                "messages": &messages,
                "max_tokens": 1024,
            }))
            .send()?;

        let response_json: serde_json::Value = response.json()?;

        let message = response_json
            .get("content")
            .and_then(|content| content.get(0))
            .and_then(|text| text.get("text"))
            .and_then(|text| text.as_str())
            .ok_or_else(|| {
                Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "Failed to generate commit message. Unexpected response format: {}",
                        response_json
                    ),
                )) as Box<dyn Error>
            })?;

        let final_message = if let Some(prefix) = prefix {
            format!("{} {}", prefix, message.trim())
        } else {
            message.trim().to_string()
        };

        Ok(final_message)
    }
}
