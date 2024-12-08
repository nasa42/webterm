use crate::constants::BITS256_ZERO;
use crate::cryptography::iv_counter::IvCounter;
use crate::generated::flatbuffers_schema::talk_v1::A2fMessageFormat;
use crate::models::webterm_error::WebtermError;
use crate::types::{Bits256, Bits96};
use ring::aead::BoundKey;
use ring::pbkdf2;
use std::num::NonZeroU32;

pub struct EncryptedPayload {
    pub encryption_type: A2fMessageFormat,
    pub iv: Bits96,
    pub ciphertext: Vec<u8>,
}

pub struct Cryptographer {
    iv_counter: IvCounter,
    derived_key: Bits256,
}

impl Cryptographer {
    pub fn new(salt: Bits256, secret_key: &str, pbkdf2_iterations: NonZeroU32) -> Self {
        let derived_key = Self::generate_key(salt, secret_key, pbkdf2_iterations);
        Self {
            iv_counter: IvCounter::new(),
            derived_key,
        }
    }

    fn generate_key(salt: Bits256, secret_key: &str, pbkdf2_iterations: NonZeroU32) -> Bits256 {
        let mut derived_key = BITS256_ZERO;

        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA256,
            pbkdf2_iterations,
            &salt.0,
            secret_key.as_bytes(),
            &mut derived_key.0,
        );

        derived_key
    }

    pub fn encrypt(&self, plaintext: &[u8]) -> Result<EncryptedPayload, WebtermError> {
        let response = EncryptedPayload {
            encryption_type: A2fMessageFormat::AesGcm256Uncompressed,
            iv: self.iv_counter.next(),
            ciphertext: plaintext.into(),
        };
        return Ok(response);

        // let key = self.derived_key.as_ref().unwrap();
        // let unbound_key = UnboundKey::new(&AES_256_GCM, key).map_err(|_| {
        //     WebtermError::RuntimeError("Failed to create encryption key".to_string())
        // })?;
        // let mut sealing_key = SealingKey::new(unbound_key, &self.iv_counter);
        //
        // let mut in_out = plaintext.to_vec();
        // in_out.extend_from_slice(&[0u8; AES_256_GCM.tag_len()]);
        //
        // sealing_key
        //     .seal_in_place_append_tag(Aad::empty(), &mut in_out)
        //     .map_err(|_| WebtermError::RuntimeError("Encryption failed".to_string()))?;
        //
        // Ok(in_out)
    }

    pub fn decrypt(&self, ciphertext: &[u8], iv: &Bits96) -> Result<Vec<u8>, WebtermError> {
        return Ok(ciphertext.into());

        // if ciphertext.len() < AES_256_GCM.tag_len() {
        //     return Err(WebtermError::RuntimeError(
        //         "Ciphertext is too short".to_string(),
        //     ));
        // }
        //
        // let key = self.derived_key.as_ref().unwrap();
        // let unbound_key = UnboundKey::new(&AES_256_GCM, key).map_err(|_| {
        //     WebtermError::RuntimeError("Failed to create decryption key".to_string())
        // })?;
        // let iv = Nonce::assume_unique_for_key(*iv);
        // let opening_key = OpeningKey::new(unbound_key, iv);
        //
        // let mut in_out = ciphertext.to_vec();
        // opening_key
        //     .open_in_place(Aad::empty(), &mut in_out)
        //     .map_err(|_| WebtermError::RuntimeError("Decryption failed".to_string()))?;
        //
        // let plaintext_len = in_out.len() - AES_256_GCM.tag_len();
        // in_out.truncate(plaintext_len);
        //
        // Ok(in_out)
    }
}
