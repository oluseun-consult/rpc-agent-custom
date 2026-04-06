use std::fmt::Display;

use serde_json::Value;

/// Represents a message to be sent to the AI provider.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Message {
    Text(String),
    Struct(Value),
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Message::Text(text) => write!(f, "{text}"),
            Message::Struct(value) => {
                if let Ok(json) = serde_json::to_string_pretty(value) {
                    write!(f, "{json}")
                } else {
                    write!(f, "{:?}", value)
                }
            }
        }
    }
}
