use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("I/O error")]
    IoError(#[from] io::Error),
    #[error("Could not resolve the given bind address")]
    BindAddressResolveError(#[source] io::Error),
    #[error("Invalid bind address")]
    InvalidBindAddress,
    #[error("Relay could not bind to the given address")]
    RelayBindError(#[source] io::Error),
    #[error("Crypto error")]
    CryptoError,
    #[error("Could not generate Ed25519 key pair")]
    Ed25519KeyPairGenerationError,
    #[error("Ed25519 key pair was rejected: {0}")]
    Ed25519KeyPairRejected(&'static str),
    #[error("Could not generate ephemeral private key")]
    PrivateKeyGenerationError,
    #[error("Could not connect to the provided relay")]
    RelayConnectionFailed,
    #[error("Connection reset")]
    ConnectionReset,
    #[error("Unexpected packet")]
    UnexpectedPacket,
    #[error("Packet format error: {0}")]
    PacketFormatError(#[from] crate::proto::PacketError),
}
