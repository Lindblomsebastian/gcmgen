use crate::client::{CommitMessageGenerator, PullRequestGenerator};
use crate::config::ServiceConfig;
use reqwest::blocking::Client;
use serde_json::{json, Value};
use std::error::Error;

pub struct OpenAIClient {
    api_key: String,
    client: Client,
    model: String,
}

impl OpenAIClient {
    pub fn new(service_config: &ServiceConfig) -> Self {
        OpenAIClient {
            api_key: service_config.api_token.clone(),
            model: service_config.model.clone(),
            client: Client::new(),
        }
    }

    pub fn generate_text(&self, messages: Value) -> Result<String, Box<dyn Error>> {
        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(&self.api_key)
            .json(&json!({
                "model": &self.model,
                "messages": messages,
                "max_tokens": 500,
            }))
            .send()?;

        let response_json: Value = response.json()?;

        response_json
            .get("choices")
            .and_then(|choices| choices.get(0))
            .and_then(|choice| choice.get("message"))
            .and_then(|content| content.get("content"))
            .and_then(|text| text.as_str())
            .map(|text| text.to_string())
            .ok_or_else(|| {
                Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "Failed to generate PR title. Unexpected response format: {}",
                        response_json
                    ),
                )) as Box<dyn Error>
            })
    }
}

impl PullRequestGenerator for OpenAIClient {
    fn generate_pr_title(
        &self,
        diff: &str,
        prefix: Option<&String>,
    ) -> Result<String, Box<dyn Error>> {
        let messages = json!([
            {
                "role": "system",
                "content": "You are a helpful assistant specialized in writing concise GitHub pull request titles."
            },
            {
                "role": "user",
                "content": format!("Here is a git diff:\n\n{}", diff)
            },
            {
                "role": "assistant",
                "content": "Generate a concise and meaningful title for a GitHub pull request based on the provided git diff."
            }
        ]);

        let title = self.generate_text(messages)?;

        let final_message = if let Some(prefix) = prefix {
            format!("{} {}", prefix, title.trim())
        } else {
            title.trim().to_string()
        };

        Ok(final_message)
    }
    fn generate_pr_description(&self, diff: &str) -> Result<String, Box<dyn Error>> {
        let messages = json!([
            {
                "role": "system",
                "content": "You are a helpful assistant specialized in writing detailed GitHub pull request descriptions."
            },
            {
                "role": "assistant",
                "content": "Generate a detailed and meaningful description for a GitHub pull request based on the provided git diff. \n\
                Only answer with the description, nothing else. The description will be read by engineers, so keep it concise and meaningful.\n\
                Don't bloat it. Keep the response under 500 tokens. \n\
                Don't include any How To's or comments about future work. \n\
                ONLY include the changes made."
            },
            {
                "role": "user",
                "content": format!("Here is a git diff:\n\n{}", diff)
            },
        ]);

        let description = self.generate_text(messages)?;

        Ok(description.trim().to_string())
    }
}

impl CommitMessageGenerator for OpenAIClient {
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

        let message = self.generate_text(messages)?;

        let final_message = if let Some(prefix) = prefix {
            format!("{} {}", prefix, message.trim())
        } else {
            message.trim().to_string()
        };

        Ok(final_message)
    }
}
