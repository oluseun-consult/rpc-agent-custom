use std::{net::SocketAddr, sync::Arc};

use rig::tool::Tool;
use schemars::JsonSchema;

use crate::{
    AgentServer, Providers,
    auth::load_private_key,
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
    script_name: Option<String>,
    function_handler: Option<String>,
    perm_file: Option<&'a str>,
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
            script_name: None,
            function_handler: None,
            perm_file: None,
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
    pub fn script_name(mut self, python_path: String) -> Self {
        self.script_name = Some(python_path);
        self
    }

    /// Sets the Python path for the provider.
    #[inline]
    pub fn function_handler(mut self, function_handler: String) -> Self {
        self.function_handler = Some(function_handler);
        self
    }

    /// Set private key for e2e.
    #[inline]
    pub fn perm_file(mut self, perm_file: &'a str) -> Self {
        self.perm_file = Some(perm_file);
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
            self.script_name,
            self.function_handler,
        )
        .await?;

        if let Some(perm_file) = self.perm_file {
            let private_key_path = load_private_key(perm_file)?;
            return Ok(AgentServer {
                socket_addr: SocketAddr::from(([0, 0, 0, 0], self.port)),
                providers: Arc::new(providers),
                private_key_path: Some(private_key_path),
            });
        }

        Ok(AgentServer {
            socket_addr: SocketAddr::from(([0, 0, 0, 0], self.port)),
            providers: Arc::new(providers),
            private_key_path: None,
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
            private_key_path: None,
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
            None,
        )
        .await?;

        Ok(AgentServer {
            socket_addr: SocketAddr::from(([0, 0, 0, 0], self.port)),
            providers: Arc::new(providers),
            private_key_path: None,
        })
    }
}
