use std::fmt::Display;

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Error type used throughout the crate.
#[derive(Error, Debug)]
pub enum Error {
    /// Provider error: the provider returned an error response.
    #[error("provider error: {0}")]
    ProviderError(String),
    /// Authentication error: the provider returned an authentication error.
    #[error("authentication error: {0}")]
    AuthenticationError(String),
    /// Client error: the HTTP client returned an error.
    #[error("client error: {0}")]
    HttpError(#[from] rig::http_client::Error),
    /// Prompt error: the prompt returned an error.
    #[error("prompt error: {0}")]
    PromptError(#[from] rig::completion::PromptError),
    /// IO error: an I/O error occurred.
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    /// RPC error: a remote procedure call error occurred.
    #[error("rpc error: {0}")]
    RpcError(#[from] tarpc::client::RpcError),
}

/// API error type used for provider-specific error responses.
#[derive(Debug, Deserialize, Serialize)]
pub struct ApiError {
    status: u16,
    message: String,
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.status, self.message)
    }
}

impl From<Error> for ApiError {
    fn from(value: Error) -> Self {
        match value {
            Error::ProviderError(e) => ApiError {
                status: 500,
                message: e,
            },
            Error::HttpError(error) => ApiError {
                status: 500,
                message: error.to_string(),
            },
            Error::PromptError(prompt_error) => ApiError {
                status: 500,
                message: prompt_error.to_string(),
            },
            Error::Io(error) => ApiError {
                status: 500,
                message: error.to_string(),
            },
            Error::AuthenticationError(e) => ApiError {
                status: 401,
                message: e,
            },
            Error::RpcError(server_error) => ApiError {
                status: 500,
                message: server_error.to_string(),
            },
        }
    }
}

impl std::error::Error for ApiError {}
