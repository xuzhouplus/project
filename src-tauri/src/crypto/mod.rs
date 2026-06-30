use aes_gcm::{
    aead::{rand_core::RngCore, Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use base64::{engine::general_purpose::STANDARD, Engine};
use keyring::Entry;
use tauri::{App, Manager};

const ENCRYPTED_PREFIX: &str = "enc:v1:";
const KEY_SERVICE: &str = "com.xuzho.project";
const KEY_USER: &str = "field-encryption-key";

pub struct CryptoState {
    cipher: Aes256Gcm,
}

impl CryptoState {
    pub fn encrypt_optional(&self, value: Option<String>) -> Result<Option<String>, String> {
        match value {
            None => Ok(None),
            Some(v) if v.is_empty() => Ok(None),
            Some(v) => Ok(Some(self.encrypt(&v)?)),
        }
    }

    pub fn decrypt_optional(&self, value: Option<String>) -> Result<Option<String>, String> {
        match value {
            None => Ok(None),
            Some(v) if v.is_empty() => Ok(None),
            Some(v) => Ok(Some(self.decrypt(&v)?)),
        }
    }

    fn encrypt(&self, plaintext: &str) -> Result<String, String> {
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let ciphertext = self
            .cipher
            .encrypt(&nonce, plaintext.as_bytes())
            .map_err(|e| format!("encryption failed: {e}"))?;

        let mut payload = nonce.to_vec();
        payload.extend(ciphertext);

        Ok(format!("{ENCRYPTED_PREFIX}{}", STANDARD.encode(payload)))
    }

    fn decrypt(&self, stored: &str) -> Result<String, String> {
        if !stored.starts_with(ENCRYPTED_PREFIX) {
            return Ok(stored.to_string());
        }

        let encoded = stored
            .strip_prefix(ENCRYPTED_PREFIX)
            .ok_or_else(|| "invalid encrypted payload".to_string())?;
        let payload = STANDARD
            .decode(encoded)
            .map_err(|e| format!("invalid encrypted payload: {e}"))?;

        if payload.len() <= 12 {
            return Err("invalid encrypted payload".to_string());
        }

        let (nonce_bytes, ciphertext) = payload.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);
        let plaintext = self
            .cipher
            .decrypt(nonce, ciphertext)
            .map_err(|_| "decryption failed".to_string())?;

        String::from_utf8(plaintext).map_err(|e| format!("invalid utf-8 in plaintext: {e}"))
    }
}

fn load_or_create_key() -> Result<[u8; 32], String> {
    let entry = Entry::new(KEY_SERVICE, KEY_USER).map_err(|e| e.to_string())?;

    match entry.get_password() {
        Ok(key_b64) => {
            let bytes = STANDARD
                .decode(key_b64)
                .map_err(|e| format!("invalid stored encryption key: {e}"))?;
            bytes
                .try_into()
                .map_err(|_| "invalid stored encryption key length".to_string())
        }
        Err(keyring::Error::NoEntry) => {
            let mut key = [0u8; 32];
            OsRng.fill_bytes(&mut key);

            entry
                .set_password(&STANDARD.encode(key))
                .map_err(|e| format!("failed to store encryption key: {e}"))?;

            Ok(key)
        }
        Err(e) => Err(format!("failed to load encryption key: {e}")),
    }
}

pub fn setup(app: &App) -> tauri::Result<()> {
    let key = load_or_create_key().map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("failed to initialize encryption: {e}"),
        )
    })?;

    let cipher = Aes256Gcm::new_from_slice(&key).map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("failed to initialize cipher: {e}"),
        )
    })?;

    app.manage(CryptoState { cipher });
    Ok(())
}
