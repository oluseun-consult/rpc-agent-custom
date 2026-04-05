use std::{net::SocketAddr, sync::Arc};

use rig::tool::Tool;
use schemars::JsonSchema;

use crate::{
    AgentServer, Providers,
    error::Error,
    tools::{NoTool, ToolWrapper},
};

pub struct AgentServerBuilder<'a> {
    port: u16,
    provider: Providers,
    system_message: &'a str,
    model: &'a str,
    api_key: Option<&'a str>,
    temperature: Option<f64>,
    max_tokens: Option<u64>,
}

impl<'a> AgentServerBuilder<'a> {
    pub fn new(port: u16, provider: Providers, system_message: &'a str, model: &'a str) -> Self {
        Self {
            port,
            provider,
            system_message,
            model,
            api_key: None,
            temperature: None,
            max_tokens: None,
        }
    }

    #[inline]
    pub fn api_key(mut self, api_key: &'a str) -> Self {
        self.api_key = Some(api_key);
        self
    }

    #[inline]
    pub fn temperature(mut self, temperature: f64) -> Self {
        self.temperature = Some(temperature);
        self
    }

    #[inline]
    pub fn max_tokens(mut self, max_tokens: u64) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    pub fn build(self) -> Result<AgentServer, Error> {
        let providers = Providers::init::<NoTool>(
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

    pub fn build_with_tool<T: Tool + 'static>(
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
        )?;

        Ok(AgentServer {
            socket_addr: SocketAddr::from(([0, 0, 0, 0], self.port)),
            providers: Arc::new(providers),
        })
    }
}
