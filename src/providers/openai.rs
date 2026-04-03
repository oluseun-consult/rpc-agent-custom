use rig::{
    agent::{Agent, AgentBuilder, WithBuilderTools},
    client::CompletionClient as _,
    completion::Prompt,
    providers::openai::{self, responses_api::ResponsesCompletionModel},
    tool::Tool,
};
use schemars::JsonSchema;

use crate::{Providers, error::Error, providers::CompletionProvider, tools::ToolWrapper};

pub struct OpenAIProvider {
    api_key: String,
    model: String,
}

impl OpenAIProvider {
    pub fn new(api_key: String, model: String) -> Self {
        Self { api_key, model }
    }

    pub fn build<T: Tool + 'static>(
        &self,
        system_message: Option<&str>,
        temperature: Option<f64>,
        max_tokens: Option<u64>,
        tool: Option<ToolWrapper<T>>,
    ) -> Result<OpenAI, Error> {
        let builder = builder(
            &self.api_key,
            &self.model,
            system_message,
            temperature,
            max_tokens,
        )?;

        let agent = if let Some(tool) = tool {
            builder_with_tools(builder, tool)?.build()
        } else {
            builder.build()
        };

        Ok(OpenAI { agent })
    }

    pub fn build_with_schema<J: JsonSchema, T: Tool + 'static>(
        &self,
        system_message: Option<&str>,
        temperature: Option<f64>,
        max_tokens: Option<u64>,
        tool: Option<ToolWrapper<T>>,
    ) -> Result<OpenAI, Error> {
        let builder = builder(
            &self.api_key,
            &self.model,
            system_message,
            temperature,
            max_tokens,
        )?
        .output_schema::<J>();

        let agent = if let Some(tool) = tool {
            builder_with_tools(builder, tool)?.build()
        } else {
            builder.build()
        };

        Ok(OpenAI { agent })
    }
}

#[derive(Clone)]
pub struct OpenAI {
    agent: Agent<ResponsesCompletionModel>,
}

impl OpenAI {
    pub fn new<T: Tool + 'static>(
        api_key: &str,
        model: &str,
        system_message: Option<&str>,
        temperature: Option<f64>,
        max_tokens: Option<u64>,
        tool: Option<ToolWrapper<T>>,
    ) -> Result<Self, Error> {
        let provider = OpenAIProvider::new(api_key.to_string(), model.to_string());

        provider.build(system_message, temperature, max_tokens, tool)
    }

    pub fn new_with_schema<J: JsonSchema, T: Tool + 'static>(
        api_key: &str,
        model: &str,
        system_message: Option<&str>,
        temperature: Option<f64>,
        max_tokens: Option<u64>,
        tool: Option<ToolWrapper<T>>,
    ) -> Result<Self, Error> {
        let provider = OpenAIProvider::new(api_key.to_string(), model.to_string());

        provider.build_with_schema::<J, T>(system_message, temperature, max_tokens, tool)
    }
}

#[async_trait::async_trait]
impl CompletionProvider for OpenAI {
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
        Providers::OpenAI
    }
}

pub fn builder(
    api_key: &str,
    model: &str,
    system_message: Option<&str>,
    temperature: Option<f64>,
    max_tokens: Option<u64>,
) -> Result<AgentBuilder<ResponsesCompletionModel>, Error> {
    let openai_client = openai::Client::new(api_key)?;

    let builder = openai_client
        .agent(model)
        .preamble(system_message.unwrap_or_default())
        .temperature(temperature.unwrap_or_default())
        .max_tokens(max_tokens.unwrap_or_default());

    Ok(builder)
}

pub fn builder_with_tools<T: Tool + 'static>(
    builder: AgentBuilder<ResponsesCompletionModel>,
    tool: ToolWrapper<T>,
) -> Result<AgentBuilder<ResponsesCompletionModel, (), WithBuilderTools>, Error> {
    let builder = builder.tool(*tool.tool());

    Ok(builder)
}
