use std::fmt::Display;

use aws_sdk_sagemakerruntime::{error::SdkError, operation::invoke_endpoint::InvokeEndpointError};
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
    /// Invalid JWT credentials: the JWT token is invalid or expired.
    #[error("invalid jwt credentials: {0}")]
    InvalidJWTCredentials(#[from] jsonwebtoken::errors::Error),
    /// No JWT secret found: the JWT secret is not configured.
    #[error("no jwt secret found")]
    NoJWTSecretFound,
    /// Invoke error: an AWS error occurred during the invoke endpoint operation.
    #[error("invoke error: an AWS error occurred during the invoke endpoint operation: {0:?}")]
    InvokeError(#[from] Box<SdkError<InvokeEndpointError>>),
    /// Serialization error: failed to serialize the request payload.
    #[error("serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

impl Error {
    fn status(&self) -> u16 {
        match self {
            Error::AuthenticationError(_) | Error::InvalidJWTCredentials(_) => 401,
            Error::HttpError(_)
            | Error::Io(_)
            | Error::PromptError(_)
            | Error::RpcError(_)
            | Error::ProviderError(_)
            | Error::NoJWTSecretFound
            | Error::InvokeError(_)
            | Error::SerializationError(_) => 500,
        }
    }
}

/// API error type used for provider-specific error responses.
#[derive(Debug, Deserialize, Serialize)]
pub struct ApiError {
    status: u16,
    message: String,
}

impl ApiError {
    pub fn status_code(&self) -> u16 {
        self.status
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.status, self.message)
    }
}

impl From<Error> for ApiError {
    fn from(value: Error) -> Self {
        Self {
            status: value.status(),
            message: value.to_string(),
        }
    }
}

impl std::error::Error for ApiError {}
