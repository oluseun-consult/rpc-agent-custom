//! # rpc-agent
//!
//! This crate provides a modular framework for building RPC-based AI agent servers.
//! It exposes core components for agent management, tool integration, error handling,
//! and provider abstraction. Use the [`AgentServerBuilder`] to configure and launch
//! your own agent server with custom tools and providers.

mod agent;
mod builder;
mod error;
mod providers;
mod tools;

/// Builder for configuring and launching an [`AgentServer`].
pub use builder::AgentServerBuilder;

/// Wrapper type for integrating tools into the agent server.
pub use tools::ToolWrapper;

/// Error type used throughout the crate.
pub use error::{ApiError, Error};

/// Main agent server type, responsible for handling requests and managing tools/providers.
pub use agent::AgentServer;

/// Enum of supported AI providers.
pub use providers::Providers;
