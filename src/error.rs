use std::fmt::Display;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("provider error: {0}")]
    ProviderError(String),
    #[error("authentication error: {0}")]
    AuthenticationError(String),
    #[error("client error: {0}")]
    HttpError(#[from] rig::http_client::Error),
    #[error("prompt error: {0}")]
    PromptError(#[from] rig::completion::PromptError),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiError {
    pub status: u16,
    pub message: String,
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
        }
    }
}

impl std::error::Error for ApiError {}
