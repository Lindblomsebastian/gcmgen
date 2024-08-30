use crate::anthropic::AnthropicClient;
use crate::config::ServiceConfig;
use crate::openai::OpenAIClient;

pub enum Client {
    OpenAI(OpenAIClient),
    Anthropic(AnthropicClient),
}

impl Client {
    pub fn new(
        service_config: &ServiceConfig,
        service_type: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        match service_type {
            "OpenAI" => Ok(Client::OpenAI(OpenAIClient::new(service_config))),
            "Anthropic" => Ok(Client::Anthropic(AnthropicClient::new(service_config))),
            _ => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Unsupported service type",
            ))),
        }
    }
}

pub trait CommitMessageGenerator {
    fn generate_commit_message(&self, diff: &str) -> Result<String, Box<dyn std::error::Error>>;
}

impl CommitMessageGenerator for Client {
    fn generate_commit_message(&self, diff: &str) -> Result<String, Box<dyn std::error::Error>> {
        match self {
            Client::OpenAI(client) => client.generate_commit_message(diff),
            Client::Anthropic(client) => client.generate_commit_message(diff),
        }
    }
}
