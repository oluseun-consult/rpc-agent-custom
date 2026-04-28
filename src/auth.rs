use std::fs;

use aes_gcm::{
    Aes256Gcm, Key, Nonce,
    aead::{Aead, KeyInit},
};
use base64::{Engine as _, engine::general_purpose};
use rsa::{Oaep, RsaPrivateKey, pkcs8::DecodePrivateKey as _, sha2::Sha256};
use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptedBundle {
    pub key: String,     // The RSA-encrypted AES key
    pub iv: String,      // The Initialization Vector for AES
    pub tag: String,     // The GCM Auth Tag (to ensure no tampering)
    pub content: String, // The actual AES-encrypted email body
}

#[cfg_attr(
    feature = "tracing",
    tracing::instrument(name = "e2e.decrypt", skip(bundle, private_key))
)]
pub fn decrypt_payload(
    bundle: EncryptedBundle,
    private_key: &RsaPrivateKey,
) -> Result<String, Error> {
    // 1. Decrypt the AES Key using RSA Private Key
    let encrypted_key = general_purpose::STANDARD.decode(bundle.key.clone())?;
    let decryptor = Oaep::new::<Sha256>();
    #[cfg(feature = "tracing")]
    tracing::info!("encrypted_key (base64): {}", &bundle.key);
    #[cfg(feature = "tracing")]
    tracing::info!("encrypted_key (len): {}", encrypted_key.len());

    let aes_raw_key = private_key.decrypt(decryptor, &encrypted_key)?;
    let _hex_encode = hex::encode(&aes_raw_key);

    #[cfg(feature = "tracing")]
    tracing::info!("aes_raw_key (hex): {}", _hex_encode);
    #[cfg(feature = "tracing")]
    tracing::info!("aes_raw_key (len): {}", aes_raw_key.len());

    // 2. Setup AES-GCM
    let key = Key::<Aes256Gcm>::from_slice(&aes_raw_key);
    let cipher = Aes256Gcm::new(key);

    let iv = general_purpose::STANDARD.decode(bundle.iv)?;
    let nonce = Nonce::from_slice(&iv);

    let mut ciphertext = general_purpose::STANDARD.decode(bundle.content.clone())?;

    #[cfg(feature = "tracing")]
    tracing::info!("ciphertext+tag (len): {}", ciphertext.len());
    #[cfg(feature = "tracing")]
    tracing::info!("ciphertext (base64): {}", &bundle.content);
    #[cfg(feature = "tracing")]
    tracing::info!("ciphertext (len): {}", ciphertext.len());

    let tag = general_purpose::STANDARD.decode(bundle.tag.clone())?;
    #[cfg(feature = "tracing")]
    tracing::info!("tag (base64): {}", &bundle.tag);
    #[cfg(feature = "tracing")]
    tracing::info!("tag (len): {}", tag.len());

    // Append tag to ciphertext
    ciphertext.extend_from_slice(&tag);
    #[cfg(feature = "tracing")]
    tracing::info!("ciphertext+tag (len): {}", ciphertext.len());

    let plaintext = cipher
        .decrypt(nonce, ciphertext.as_ref())
        .map_err(|e| Error::CipherDecryptError(crate::error::CipherAeadError(e)))?;

    #[cfg(feature = "tracing")]
    tracing::info!("plaintext (utf8?): {}", String::from_utf8_lossy(&plaintext));

    String::from_utf8(plaintext).map_err(|e| Error::ProviderError(e.to_string()))
}

pub fn load_private_key(pem_path: &str) -> Result<RsaPrivateKey, Error> {
    // Read the pem file from your project root or a secure path
    let pem_data = fs::read_to_string(pem_path)?;

    // Parse the PEM string into a usable RSA Private Key
    RsaPrivateKey::from_pkcs8_pem(&pem_data).map_err(Error::from)
}
