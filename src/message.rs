use serde_json::Value;

use crate::error::{ApiError, Error};
use crate::jwt::decode_jwt;

/// Represents a message to be sent to the AI provider.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Message {
    /// A plain text message.
    Text(String),
    /// A structured message.
    Struct(Value),
    /// A JWT-backed message whose claims will be validated and whose prompt will be extracted
    /// before being sent.
    ///
    /// Note: This does not provide confidentiality by itself; it relies on `JWT_SECRET` to
    /// validate/decode the token.
    Encrypted(String),
}

impl TryFrom<Message> for String {
    type Error = ApiError;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        match message {
            Message::Text(text) => Ok(text),
            Message::Struct(value) => {
                if let Ok(json) = serde_json::to_string_pretty(&value) {
                    Ok(json)
                } else {
                    Ok(value.to_string())
                }
            }
            Message::Encrypted(token) => {
                let hmac_secret =
                    std::env::var("JWT_SECRET").map_err(|_| Error::NoJWTSecretFound)?;

                let prompt = decode_jwt(&token, &hmac_secret).map(|c| c.prompt)?;

                Ok(prompt)
            }
        }
    }
}
