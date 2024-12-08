use crate::config::DEFAULT_PBKDF2_ITERATIONS;
use crate::models::agent_error::AgentError;
use crate::models::session::Session;
use std::num::NonZeroU32;
use std::sync::Arc;
use tokio::sync::Mutex;
use webterm_core::constants::{BITS256_ZERO, BITS96_ZERO};
use webterm_core::cryptography::cryptographer::Cryptographer;
use webterm_core::random::random_bytes;
use webterm_core::types::{Bits256, Bits96, FrontendId};

pub struct Frontend {
    frontend_id: FrontendId,
    session: Option<Arc<Mutex<Session>>>,
    salt: Bits256,
    cryptographer: Option<Cryptographer>,
    pbkdf2_iterations: u32,
    challenge_nonce: Option<(Bits96, Bits256)>,
}

impl Frontend {
    pub fn new(frontend_id: FrontendId) -> Self {
        let mut salt = BITS256_ZERO;
        random_bytes(&mut salt.0);

        // TODO: Guarantee that it will never be reused in IvCounter for this session
        let mut challenge_iv = BITS96_ZERO;
        random_bytes(&mut challenge_iv.0);

        let mut challenge_nonce = BITS256_ZERO;
        random_bytes(&mut challenge_nonce.0);

        Self {
            frontend_id,
            session: None,
            salt,
            cryptographer: None,
            pbkdf2_iterations: DEFAULT_PBKDF2_ITERATIONS,
            challenge_nonce: Some((challenge_iv, challenge_nonce)),
        }
    }

    pub fn frontend_id(&self) -> FrontendId {
        self.frontend_id
    }

    pub fn register_session(&mut self, session: Arc<Mutex<Session>>) {
        self.session = Some(session);
    }

    pub async fn session(&self) -> Result<Arc<Mutex<Session>>, AgentError> {
        Ok(self
            .session
            .as_ref()
            .ok_or(AgentError::SessionNotFound(None))?
            .clone())
    }

    pub fn init_cryptographer(&mut self, secret_key: &str) {
        let iterations = NonZeroU32::new(self.pbkdf2_iterations).unwrap();

        self.cryptographer = Some(Cryptographer::new(self.salt, secret_key, iterations))
    }

    pub fn cryptographer(&self) -> Result<&Cryptographer, AgentError> {
        self.cryptographer.as_ref().ok_or(AgentError::RuntimeError(
            "Cryptographer is not initialised".to_string(),
        ))
    }

    pub fn verify_and_consume_nonce(
        &mut self,
        encrypted_nonce: &Bits256,
        iv: &Bits96,
    ) -> Result<bool, AgentError> {
        if let Some((_challenge_iv, challenge_nonce)) = (self.challenge_nonce) {
            let decrypted_nonce = self
                .cryptographer()?
                .decrypt(encrypted_nonce.0.as_ref(), iv)?;
            let result = challenge_nonce.0.as_ref() == &decrypted_nonce;
            self.challenge_nonce = None;
            Ok(result)
        } else {
            return Err(AgentError::RuntimeError(
                "Challenge nonce is not set".to_string(),
            ));
        }
    }

    pub fn salt(&self) -> Bits256 {
        self.salt
    }

    pub fn pbkdf2_iterations(&self) -> u32 {
        self.pbkdf2_iterations
    }

    pub fn challenge_iv(&self) -> Result<Bits96, AgentError> {
        Ok(self
            .challenge_nonce
            .ok_or(AgentError::RuntimeError(
                "Challenge nonce is not set".to_string(),
            ))?
            .0)
    }

    pub fn challenge_nonce(&self) -> Result<Bits256, AgentError> {
        Ok(self
            .challenge_nonce
            .ok_or(AgentError::RuntimeError(
                "Challenge nonce is not set".to_string(),
            ))?
            .1)
    }
}
