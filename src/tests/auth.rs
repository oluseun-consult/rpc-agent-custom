use crate::auth::{EncryptedBundle, decrypt_payload, load_private_key};
use aes_gcm::AeadCore;
use aes_gcm::{
    Aes256Gcm,
    aead::{Aead, KeyInit},
};
use base64::{Engine as _, engine::general_purpose};
use rand::rngs::OsRng;
use rsa::traits::PublicKeyParts;
use rsa::{Oaep, RsaPrivateKey, RsaPublicKey, pkcs8::EncodePrivateKey};
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_load_private_key() {
    // Generate a new private key for testing
    let mut rng = OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, 2048).unwrap();
    let pem = private_key.to_pkcs8_pem(Default::default()).unwrap();
    let mut tmpfile = NamedTempFile::new().unwrap();
    write!(tmpfile, "{}", pem.as_str()).unwrap();
    let path = tmpfile.path().to_str().unwrap();
    let loaded = load_private_key(path).unwrap();
    assert_eq!(private_key.n(), loaded.n());
}

#[test]
fn test_decrypt_payload() {
    // Generate RSA keypair
    let mut rng = OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, 2048).unwrap();
    let public_key = RsaPublicKey::from(&private_key);

    // Generate AES key and IV
    let aes_key = Aes256Gcm::generate_key(&mut rng);
    let iv = Aes256Gcm::generate_nonce(&mut rng);
    let cipher = Aes256Gcm::new(&aes_key);

    // Encrypt plaintext
    let plaintext = b"hello world";
    let ciphertext = cipher.encrypt(&iv, plaintext.as_ref()).unwrap();

    // Encrypt AES key with RSA
    let encryptor = Oaep::new::<rsa::sha2::Sha256>();
    let encrypted_key = public_key
        .encrypt(&mut rng, encryptor, aes_key.as_slice())
        .unwrap();

    // Prepare EncryptedBundle
    let bundle = EncryptedBundle {
        key: general_purpose::STANDARD.encode(&encrypted_key),
        iv: general_purpose::STANDARD.encode(&iv),
        tag: String::new(), // Not used in this implementation
        content: general_purpose::STANDARD.encode(&ciphertext),
    };

    // Decrypt
    let result = decrypt_payload(bundle, &private_key).unwrap();
    assert_eq!(result, "hello world");
}
