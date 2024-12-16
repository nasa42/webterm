use crate::compress::{compress, decompress};
use crate::cryptography::iv_counter::IvCounter;
use crate::models::webterm_error::WebtermError;
use crate::types::{Bits256, Bits96};
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use pbkdf2::pbkdf2_hmac_array;
use sha2::Sha256;

pub struct EncryptedPayload {
    pub iv: Bits96,
    pub ciphertext: Vec<u8>,
    pub compressed: bool,
}

pub struct Cryptographer {
    iv_counter: IvCounter,
    derived_key: Bits256,
}

const COMPRESSION_THRESHOLD: usize = 512;

impl Cryptographer {
    pub fn new(salt: Bits256, secret_key: &str, pbkdf2_iterations: u32) -> Self {
        let derived_key = Self::generate_key(salt, secret_key, pbkdf2_iterations);
        Self {
            iv_counter: IvCounter::new(),
            derived_key,
        }
    }

    fn generate_key(salt: Bits256, secret_key: &str, pbkdf2_iterations: u32) -> Bits256 {
        let derived_key =
            pbkdf2_hmac_array::<Sha256, 32>(secret_key.as_bytes(), &salt.0, pbkdf2_iterations);
        Bits256(derived_key)
    }

    pub fn encrypt(
        &self,
        plaintext: &[u8],
        may_compress: bool,
    ) -> Result<EncryptedPayload, WebtermError> {
        let cipher = Aes256Gcm::new_from_slice(&self.derived_key.0).map_err(|_| {
            WebtermError::RuntimeError("Failed to create encryption key".to_string())
        })?;

        let iv = self.iv_counter.next();
        let nonce = Nonce::from_slice(iv.0.as_ref());

        let (payload, compressed) = if may_compress && plaintext.len() > COMPRESSION_THRESHOLD {
            (compress(plaintext)?, true)
        } else {
            (plaintext.to_vec(), false)
        };

        if (compressed) {
            println!("compressed payload is: {:?} ", payload);
        }

        let ciphertext = cipher
            .encrypt(nonce, payload.as_ref())
            .map_err(|_| WebtermError::EncryptionError("Encryption failed".to_string()))?;

        Ok(EncryptedPayload {
            ciphertext,
            iv,
            compressed,
        })
    }

    pub fn decrypt(
        &self,
        ciphertext: &[u8],
        iv: &Bits96,
        compressed: bool,
    ) -> Result<Vec<u8>, WebtermError> {
        let cipher = Aes256Gcm::new_from_slice(&self.derived_key.0).map_err(|_| {
            WebtermError::RuntimeError("Failed to create decryption key".to_string())
        })?;

        let nonce = Nonce::from_slice(iv.0.as_ref());

        let decrypted = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|_| WebtermError::DecryptionError("Decryption failed".to_string()))?;

        if compressed {
            decompress(&decrypted)
        } else {
            Ok(decrypted)
        }
    }
}
