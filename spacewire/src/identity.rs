use ring::{
    rand,
    signature::{self, Ed25519KeyPair, KeyPair},
};

use crate::Error;

#[derive(Debug)]
pub struct Identity {
    key_pair: Ed25519KeyPair,
}

/// Generates a new Ed25519 key pair using the system RNG.
fn generate_keypair() -> Result<signature::Ed25519KeyPair, Error> {
    let rng = rand::SystemRandom::new();
    let pkcs8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng)
        .map_err(|_| Error::Ed25519KeyPairGenerationError)?;
    let key_pair = signature::Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref())
        .map_err(|e| Error::Ed25519KeyPairRejected(e.description_()))?;

    Ok(key_pair)
}

impl Identity {
    /// Constructs a new [`Identity`] with a newly generated identity key.
    pub fn new() -> Result<Identity, Error> {
        let key_pair = generate_keypair()?;

        Ok(Identity { key_pair })
    }

    /// Returns the public key part of the identity key.
    pub fn public_key(&self) -> &[u8] {
        self.key_pair.public_key().as_ref()
    }
}
