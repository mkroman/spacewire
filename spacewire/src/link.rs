use log::trace;
use ring::{agreement, rand};
use tokio::net::{self, TcpStream, ToSocketAddrs};

use crate::Error;

#[derive(Debug)]
pub struct EphemeralLink {
    private_key: agreement::EphemeralPrivateKey,
}

impl EphemeralLink {
    /// Creates a new link with a new, securely generated ephemeral private key.
    pub fn new() -> Result<EphemeralLink, Error> {
        let rng = rand::SystemRandom::new();
        let private_key = agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng)
            .map_err(|_| Error::PrivateKeyGenerationError)?;

        Ok(EphemeralLink { private_key })
    }

    /// Computes and returns the public key for this ephemeral link.
    pub fn public_key(&self) -> Result<agreement::PublicKey, Error> {
        self.private_key
            .compute_public_key()
            .map_err(|_| Error::CryptoError)
    }

    /// Connects to the given relay at `addr`.
    pub async fn connect<T: ToSocketAddrs>(&self, addrs: T) -> Result<(), Error> {
        // Resolve the address and find one that is connectable
        let addrs = net::lookup_host(addrs).await?;

        for addr in addrs {
            if let Ok(stream) = TcpStream::connect(addr).await {
                trace!("Connected to relay {}", stream.peer_addr()?);

                return Ok(());
            }
        }

        Err(Error::RelayConnectionFailed)
    }
}
