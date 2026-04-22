use std::{net::SocketAddr, sync::Arc};

use rig::tool::Tool;
use schemars::JsonSchema;

use crate::{
    AgentServer, Providers,
    error::Error,
    tools::{NoTool, ToolWrapper},
};

/// Builder for creating an [`AgentServer`].
pub struct AgentServerBuilder<'a> {
    port: u16,
    provider: Providers,
    system_message: &'a str,
    model: &'a str,
    api_key: Option<&'a str>,
    temperature: Option<f64>,
    max_tokens: Option<u64>,
    python_path: Option<String>,
}

impl<'a> AgentServerBuilder<'a> {
    /// Creates a new [`AgentServerBuilder`] with the given port, provider, system message, and model.
    pub fn new(port: u16, provider: Providers, system_message: &'a str, model: &'a str) -> Self {
        Self {
            port,
            provider,
            system_message,
            model,
            api_key: None,
            temperature: None,
            max_tokens: None,
            python_path: None,
        }
    }

    /// Sets the API key for the provider.
    #[inline]
    pub fn api_key(mut self, api_key: &'a str) -> Self {
        self.api_key = Some(api_key);
        self
    }

    /// Sets the temperature for the provider.
    #[inline]
    pub fn temperature(mut self, temperature: f64) -> Self {
        self.temperature = Some(temperature);
        self
    }

    /// Sets the maximum number of tokens for the provider.
    #[inline]
    pub fn max_tokens(mut self, max_tokens: u64) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    /// Sets the Python path for the provider.
    #[inline]
    pub fn python_path(mut self, python_path: String) -> Self {
        self.python_path = Some(python_path);
        self
    }

    /// Builds the [`AgentServer`] with the given configuration.
    pub async fn build(self) -> Result<AgentServer, Error> {
        let providers = Providers::init::<NoTool>(
            self.provider,
            self.model,
            self.api_key,
            self.system_message.to_string(),
            self.temperature,
            self.max_tokens,
            None,
            self.python_path,
        )
        .await?;

        Ok(AgentServer {
            socket_addr: SocketAddr::from(([0, 0, 0, 0], self.port)),
            providers: Arc::new(providers),
        })
    }

    /// Builds the [`AgentServer`] with the given configuration and schema.
    pub fn build_with_schema<J: JsonSchema>(self) -> Result<AgentServer, Error> {
        let providers = Providers::init_with_schema::<J, NoTool>(
            self.provider,
            self.model,
            self.api_key,
            self.system_message.to_string(),
            self.temperature,
            self.max_tokens,
            None,
        )?;

        Ok(AgentServer {
            socket_addr: SocketAddr::from(([0, 0, 0, 0], self.port)),
            providers: Arc::new(providers),
        })
    }

    /// Builds the [`AgentServer`] with the given configuration and tool.
    pub async fn build_with_tool<T: Tool + 'static>(
        self,
        tool: ToolWrapper<T>,
    ) -> Result<AgentServer, Error> {
        let providers = Providers::init::<T>(
            self.provider,
            self.model,
            self.api_key,
            self.system_message.to_string(),
            self.temperature,
            self.max_tokens,
            Some(tool),
            None,
        )
        .await?;

        Ok(AgentServer {
            socket_addr: SocketAddr::from(([0, 0, 0, 0], self.port)),
            providers: Arc::new(providers),
        })
    }
}
