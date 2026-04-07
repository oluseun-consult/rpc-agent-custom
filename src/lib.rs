//! # rpc-agent
//!
//! This crate provides a modular framework for building RPC-based AI agent servers.
//! It exposes core components for agent management, tool integration, error handling,
//! and provider abstraction. Use the [`AgentServerBuilder`] to configure and launch
//! your own agent server with custom tools and providers.
//!
//! ## Example
//!
//! Here is a minimal example of how to set up and run an Ollama RPC agent:
//!
//!```rust,ignore
//!use rpc_agent::Providers;
//!
//!#[tokio::main]
//!async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!    let builder = rpc_agent::AgentServerBuilder::new(
//!        5500,
//!        Providers::Ollama,
//!        "You're a friendly assistant",
//!        "gpt-oss:20b",
//!    );
//!
//!    let server = builder.build()?;
//!
//!    server.run().await?;
//!
//!    Ok(())
//!}
//!```

mod agent;
mod builder;
pub mod error;
mod jwt;

mod message;
mod providers;
mod tools;

#[cfg(test)]
mod tests;

/// Builder for configuring and launching an [`AgentServer`].
pub use builder::AgentServerBuilder;

/// Wrapper type for integrating tools into the agent server.
pub use tools::ToolWrapper;

/// Main agent server type, responsible for handling requests and managing tools/providers.
pub use agent::AgentServer;

/// Enum of supported AI providers.
pub use providers::Providers;

pub use message::Message;
