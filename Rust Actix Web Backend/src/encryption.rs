use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use base64::{engine::general_purpose, Engine as _};

fn get_encryption_key() -> Key<Aes256Gcm> {
    let key_str = std::env::var("ENCRYPTION_KEY").expect("ENCRYPTION_KEY Must be Set");

    // Key must be exactly 32 bytes for AES-256
    let key_bytes = general_purpose::STANDARD
        .decode(&key_str)
        .expect("ENCRYPTION_KEY Must be Valid Base64");

    assert!(key_bytes.len() == 32, "ENCRYPTION_KEY Must be 32 Bytes");

    *Key::<Aes256Gcm>::from_slice(&key_bytes)
}

pub fn encrypt_message(plaintext: &str) -> Result<String, String> {
    let key = get_encryption_key();
    let cipher = Aes256Gcm::new(&key);

    // Generate random nonce — different for every message
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    match cipher.encrypt(&nonce, plaintext.as_bytes()) {
        Ok(ciphertext) => {
            // Combine nonce + ciphertext then base64 encode
            let mut combined = nonce.to_vec();
            combined.extend_from_slice(&ciphertext);
            Ok(general_purpose::STANDARD.encode(&combined))
        }
        Err(e) => Err(format!("Encryption failed: {}", e)),
    }
}

pub fn decrypt_message(encrypted: &str) -> Result<String, String> {
    let key = get_encryption_key();
    let cipher = Aes256Gcm::new(&key);

    // Decode base64
    let combined = match general_purpose::STANDARD.decode(encrypted) {
        Ok(combined_str) => combined_str,
        Err(e) => return Err(format!("Base64 decode failed: {}", e)),
    };

    // Split nonce (first 12 bytes) from ciphertext
    if combined.len() < 12 {
        return Err("Invalid encrypted message".to_string());
    }

    let (nonce_bytes, ciphertext) = combined.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);

    match cipher.decrypt(nonce, ciphertext) {
        Ok(plaintext) => {
            String::from_utf8(plaintext).map_err(|e| format!("UTF-8 decode failed: {}", e))
        }
        Err(e) => Err(format!("Decryption failed: {}", e)),
    }
}
