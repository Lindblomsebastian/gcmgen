use reqwest::blocking::Client;
use serde_json::json;
use std::error::Error;

pub struct OpenAIClient {
    api_key: String,
    client: Client,
    pub model: String,
}

impl OpenAIClient {
    pub fn new(api_key: &str) -> Self {
        OpenAIClient {
            api_key: api_key.to_string(),
            client: Client::new(),
            model: "gpt-4o-mini".to_string(), // default model
        }
    }

    pub fn with_model(mut self, model: &str) -> Self {
        self.model = String::from(model);
        self
    }

    pub fn generate_commit_message(&self, diff: &str) -> Result<String, Box<dyn Error>> {
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
            .post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(&self.api_key)
            .json(&json!({
                "model": &self.model,
                "messages": messages,
                "max_tokens": 60,
            }))
            .send()?;

        let response_json: serde_json::Value = response.json()?;

        let message = response_json
            .get("choices")
            .and_then(|choices| choices.get(0))
            .and_then(|choice| choice.get("message"))
            .and_then(|content| content.get("content"))
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

        Ok(message.trim().to_string())
    }
}
