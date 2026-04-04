use std::fmt::Display;

use rig::tool::Tool;
use schemars::JsonSchema;

use crate::{ToolWrapper, error::Error};

mod ollama;
mod openai;

#[async_trait::async_trait]
pub trait CompletionProvider: Send + Sync {
    async fn chat(&self, prompt: &str) -> Result<String, Error>;
    fn model(&self) -> &str;
    fn api_key(&self) -> Option<&str>;
    fn system_message(&self) -> Option<&str>;
    fn temperature(&self) -> Option<f64>;
    fn max_tokens(&self) -> Option<u64>;
    fn provider(&self) -> Providers;
}

/// Supported AI providers for the agent server.
pub enum Providers {
    Ollama,
    OpenAI,
}

impl Display for Providers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Providers::Ollama => write!(f, "ollama"),
            Providers::OpenAI => write!(f, "openai"),
        }
    }
}

impl From<&str> for Providers {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "ollama" => Providers::Ollama,
            "openai" => Providers::OpenAI,
            _ => panic!(
                "unknown provider: {}. Currently supported providers are: ollama, openai",
                value
            ),
        }
    }
}

impl Providers {
    pub fn init<T: Tool + 'static>(
        provider: Providers,
        model: &str,
        api_key: Option<&str>,
        system_message: String,
        temperature: Option<f64>,
        max_tokens: Option<u64>,
        tool: Option<ToolWrapper<T>>,
    ) -> Result<Box<dyn CompletionProvider>, Error> {
        match provider {
            Providers::Ollama => {
                let client = ollama::OllamaAI::new(
                    model,
                    Some(&system_message),
                    temperature,
                    max_tokens,
                    tool,
                )?;
                Ok(Box::new(client))
            }
            Providers::OpenAI => {
                let api_key = api_key.ok_or_else(|| {
                    Error::AuthenticationError(
                        "api_key is required for openai provider".to_string(),
                    )
                })?;

                let client = openai::OpenAI::new(
                    api_key,
                    model,
                    Some(&system_message),
                    temperature,
                    max_tokens,
                    tool,
                )?;
                Ok(Box::new(client))
            }
        }
    }

    pub fn init_with_schema<J: JsonSchema, T: Tool + 'static>(
        provider: Providers,
        model: &str,
        api_key: Option<&str>,
        system_message: String,
        temperature: Option<f64>,
        max_tokens: Option<u64>,
        tool: Option<ToolWrapper<T>>,
    ) -> Result<Box<dyn CompletionProvider>, Error> {
        match provider {
            Providers::Ollama => {
                let client = ollama::OllamaAI::new_with_schema::<J, T>(
                    model,
                    Some(&system_message),
                    temperature,
                    max_tokens,
                    tool,
                )?;
                Ok(Box::new(client))
            }
            Providers::OpenAI => {
                let api_key = api_key.ok_or_else(|| {
                    Error::AuthenticationError(
                        "api_key is required for openai provider".to_string(),
                    )
                })?;

                let client = openai::OpenAI::new_with_schema::<J, T>(
                    api_key,
                    model,
                    Some(&system_message),
                    temperature,
                    max_tokens,
                    tool,
                )?;
                Ok(Box::new(client))
            }
        }
    }
}
