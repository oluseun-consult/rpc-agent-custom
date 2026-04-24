use rsa::RsaPrivateKey;
use serde_json::Value;

use crate::auth::{EncryptedBundle, decrypt_payload};
use crate::error::{ApiError, Error};

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

impl Message {
    pub fn to_string(self, private_key: RsaPrivateKey) -> Result<String, ApiError> {
        match self {
            Message::Text(text) => Ok(text),
            Message::Struct(value) => {
                if let Ok(json) = serde_json::to_string_pretty(&value) {
                    Ok(json)
                } else {
                    Ok(value.to_string())
                }
            }
            Message::Encrypted(token) => {
                let bundle: EncryptedBundle = serde_json::from_str(&token)
                    .map_err(|e| ApiError::from(Error::SerializationError(e)))?;

                let prompt = decrypt_payload(bundle, &private_key)?;

                Ok(prompt)
            }
        }
    }
}
