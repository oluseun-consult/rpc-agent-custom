use rig::{
    agent::{Agent, AgentBuilder, WithBuilderTools},
    client::{CompletionClient as _, Nothing},
    completion::Prompt as _,
    providers::ollama::{self, CompletionModel},
    tool::Tool,
};
use schemars::JsonSchema;

use crate::{Providers, error::Error, providers::CompletionProvider, tools::ToolWrapper};

pub struct OllamaProvider {
    model: String,
}

impl OllamaProvider {
    pub fn new(model: String) -> Self {
        Self { model }
    }

    pub fn build<T: Tool + 'static>(
        &self,
        system_message: Option<&str>,
        temperature: Option<f64>,
        max_tokens: Option<u64>,
        tool: Option<ToolWrapper<T>>,
    ) -> Result<OllamaAI, Error> {
        let builder = builder(&self.model, system_message, temperature, max_tokens)?;

        let agent = if let Some(tool) = tool {
            builder_with_tools(builder, tool)?.build()
        } else {
            builder.build()
        };

        Ok(OllamaAI { agent })
    }

    pub fn build_with_schema<J: JsonSchema, T: Tool + 'static>(
        &self,
        system_message: Option<&str>,
        temperature: Option<f64>,
        max_tokens: Option<u64>,
        tool: Option<ToolWrapper<T>>,
    ) -> Result<OllamaAI, Error> {
        let builder =
            builder(&self.model, system_message, temperature, max_tokens)?.output_schema::<J>();

        let agent = if let Some(tool) = tool {
            builder_with_tools(builder, tool)?.build()
        } else {
            builder.build()
        };

        Ok(OllamaAI { agent })
    }
}

#[derive(Clone)]
pub struct OllamaAI {
    pub agent: Agent<CompletionModel>,
}

impl OllamaAI {
    pub fn new<T: Tool + 'static>(
        model: &str,
        system_message: Option<&str>,
        temperature: Option<f64>,
        max_tokens: Option<u64>,
        tool: Option<ToolWrapper<T>>,
    ) -> Result<Self, Error> {
        let provider = OllamaProvider::new(model.to_string());

        provider.build(system_message, temperature, max_tokens, tool)
    }

    pub fn new_with_schema<J: JsonSchema, T: Tool + 'static>(
        model: &str,
        system_message: Option<&str>,
        temperature: Option<f64>,
        max_tokens: Option<u64>,
        tool: Option<ToolWrapper<T>>,
    ) -> Result<Self, Error> {
        let provider = OllamaProvider::new(model.to_string());

        provider.build_with_schema::<J, T>(system_message, temperature, max_tokens, tool)
    }
}

#[async_trait::async_trait]
impl CompletionProvider for OllamaAI {
    async fn chat(&self, prompt: &str) -> Result<String, Error> {
        let response = self.agent.prompt(prompt).await?;
        Ok(response)
    }
    fn model(&self) -> &str {
        &self.agent.model.model
    }
    fn api_key(&self) -> Option<&str> {
        None
    }
    fn system_message(&self) -> Option<&str> {
        self.agent.preamble.as_deref()
    }
    fn temperature(&self) -> Option<f64> {
        self.agent.temperature
    }
    fn max_tokens(&self) -> Option<u64> {
        self.agent.max_tokens
    }
    fn provider(&self) -> Providers {
        Providers::Ollama
    }
}

pub fn builder(
    model: &str,
    system_message: Option<&str>,
    temperature: Option<f64>,
    max_tokens: Option<u64>,
) -> Result<AgentBuilder<CompletionModel>, Error> {
    let openai_client = ollama::Client::new(Nothing)?;

    let builder = openai_client
        .agent(model)
        .preamble(system_message.unwrap_or_default())
        .temperature(temperature.unwrap_or_default())
        .max_tokens(max_tokens.unwrap_or_default());

    Ok(builder)
}

pub fn builder_with_tools<T: Tool + 'static>(
    builder: AgentBuilder<CompletionModel>,
    tool: ToolWrapper<T>,
) -> Result<AgentBuilder<CompletionModel, (), WithBuilderTools>, Error> {
    let builder = builder.tool(*tool.tool());

    Ok(builder)
}
