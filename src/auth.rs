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
    let encrypted_key = general_purpose::STANDARD.decode(bundle.key)?;
    let decryptor = Oaep::new::<Sha256>();

    #[cfg(feature = "tracing")]
    tracing::info!("Decrypting starts");

    let aes_raw_key = private_key.decrypt(decryptor, &encrypted_key)?;

    #[cfg(feature = "tracing")]
    tracing::info!("Decrypting done");

    // 2. Setup AES-GCM
    let key = Key::<Aes256Gcm>::from_slice(&aes_raw_key);
    let cipher = Aes256Gcm::new(key);

    let iv = general_purpose::STANDARD.decode(bundle.iv)?;
    let nonce = Nonce::from_slice(&iv);

    let ciphertext = general_purpose::STANDARD.decode(bundle.content)?;
    // (Note: In AES-GCM, the 'tag' is usually appended to the ciphertext)

    // 3. Decrypt the email body
    let plaintext = cipher
        .decrypt(nonce, ciphertext.as_ref())
        .map_err(|e| Error::CipherDecryptError(crate::error::CipherAeadError(e)))?;

    String::from_utf8(plaintext).map_err(|e| Error::ProviderError(e.to_string()))
}

pub fn load_private_key(pem_path: &str) -> Result<RsaPrivateKey, Error> {
    // Read the pem file from your project root or a secure path
    let pem_data = fs::read_to_string(pem_path)?;

    // Parse the PEM string into a usable RSA Private Key
    RsaPrivateKey::from_pkcs8_pem(&pem_data).map_err(Error::from)
}
