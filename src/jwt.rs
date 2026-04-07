use jsonwebtoken::Algorithm;
use serde::{Deserialize, Serialize};

use crate::error::Error;

/// JWT token decoder
/// Decodes a JWT token and returns the claims if valid.
#[tracing::instrument(name = "jwt.decode", skip(token, hmac_secret))]
pub(crate) fn decode_jwt(token: &str, hmac_secret: &str) -> Result<Claims, Error> {
    let validation = jsonwebtoken::Validation::new(Algorithm::HS256);

    jsonwebtoken::decode::<Claims>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(hmac_secret.as_bytes()),
        &validation,
    )
    .map(|data| data.claims)
    .map_err(Error::InvalidJWTCredentials)
}

/// JWT claims
#[derive(Deserialize, Serialize)]
pub(crate) struct Claims {
    pub prompt: String,
    pub exp: i64,
}
