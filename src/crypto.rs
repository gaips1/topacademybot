use aes_gcm::{
    Aes256Gcm, Key, Nonce,
    aead::{Aead, AeadCore, KeyInit, OsRng},
};
use base64::prelude::*;
use sha2::{Digest, Sha256};
use std::sync::LazyLock;

const ENCRYPTED_PREFIX: &str = "enc:";

static CIPHER: LazyLock<Aes256Gcm> = LazyLock::new(|| {
    let secret = std::env::var("ENCRYPTION_KEY").expect("no ENCRYPTION_KEY");
    let key_bytes = Sha256::digest(secret.as_bytes());
    Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key_bytes))
});

pub fn encrypt_password(plaintext: &str) -> Result<String, String> {
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = CIPHER
        .encrypt(&nonce, plaintext.as_bytes())
        .map_err(|e| format!("Ошибка шифрования: {e}"))?;

    let mut combined = Vec::with_capacity(nonce.len() + ciphertext.len());
    combined.extend_from_slice(&nonce);
    combined.extend_from_slice(&ciphertext);

    let b64 = BASE64_STANDARD.encode(&combined);
    Ok(format!("{ENCRYPTED_PREFIX}{b64}"))
}

pub fn decrypt_password(stored: &str) -> Result<String, String> {
    let Some(b64) = stored.strip_prefix(ENCRYPTED_PREFIX) else {
        return Ok(stored.to_string());
    };

    let combined = BASE64_STANDARD
        .decode(b64)
        .map_err(|e| format!("Ошибка декодирования Base64: {e}"))?;

    if combined.len() < 12 {
        return Err("Некорректная длина шифртекста".to_string());
    }

    let (nonce_bytes, ciphertext) = combined.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);

    let decrypted = CIPHER
        .decrypt(nonce, ciphertext)
        .map_err(|e| format!("Ошибка расшифровки: {e}"))?;

    String::from_utf8(decrypted).map_err(|e| format!("Ошибка UTF-8: {e}"))
}
