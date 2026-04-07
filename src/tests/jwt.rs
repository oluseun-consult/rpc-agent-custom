use crate::error::{ApiError, Error};
use crate::jwt::{Claims, decode_jwt};
use crate::message::Message;
use std::env;

/// Guard to ensure JWT_SECRET is removed from the environment after test, even on panic.
struct JwtSecretGuard {
    previous: Option<String>,
}
impl JwtSecretGuard {
    fn set(secret: &str) -> Self {
        let previous = env::var("JWT_SECRET").ok();
        unsafe { env::set_var("JWT_SECRET", secret) };
        JwtSecretGuard { previous }
    }

    fn delete() -> Self {
        let previous = env::var("JWT_SECRET").ok();
        unsafe { env::remove_var("JWT_SECRET") };
        JwtSecretGuard { previous }
    }
}
impl Drop for JwtSecretGuard {
    fn drop(&mut self) {
        match &self.previous {
            Some(previous) => unsafe { env::set_var("JWT_SECRET", previous) },
            None => unsafe { env::remove_var("JWT_SECRET") },
        }
    }
}

#[test]
#[serial_test::serial]
fn decode_jwt_expired_token() {
    let secret = "mysecret";
    let claims = Claims {
        prompt: "expired prompt".to_string(),
        exp: 1, // Expired long ago (Unix epoch)
    };
    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap();
    let result = decode_jwt(&token, secret);
    assert!(matches!(result, Err(Error::InvalidJWTCredentials(_))));
}

#[test]
#[serial_test::serial]
fn decode_jwt_valid_token() {
    let secret = "mysecret";
    let claims = Claims {
        prompt: "test prompt".to_string(),
        exp: 9999999999,
    };
    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap();
    let decoded = decode_jwt(&token, secret).unwrap();
    assert_eq!(decoded.prompt, "test prompt");
}

#[test]
fn decode_jwt_invalid_token() {
    let secret = "mysecret";
    let token = "invalid.token.value";
    let result = decode_jwt(token, secret);
    assert!(matches!(result, Err(Error::InvalidJWTCredentials(_))));
}

#[test]
#[serial_test::serial]
fn message_encrypted_variant_decrypts() {
    let secret = "testsecret";
    let _guard = JwtSecretGuard::set(secret);
    let claims = Claims {
        prompt: "encrypted prompt".to_string(),
        exp: 9999999999,
    };
    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap();
    let msg = Message::Encrypted(token);
    let result: String = msg.try_into().unwrap();
    assert_eq!(result, "encrypted prompt");
}

#[test]
#[serial_test::serial]
fn message_encrypted_variant_no_secret() {
    let _guard = JwtSecretGuard::delete();
    let msg = Message::Encrypted("sometoken".to_string());
    let result: Result<String, ApiError> = msg.try_into();
    assert_eq!(
        result.unwrap_err().to_string(),
        ApiError::from(Error::NoJWTSecretFound).to_string()
    );
}
