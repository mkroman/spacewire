use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error")]
    IoError(#[from] io::Error),
    #[error("Could not resolve the given bind address")]
    BindAddressResolveError(#[source] io::Error),
    #[error("Invalid bind address")]
    InvalidBindAddress,
    #[error("Relay could not bind to the given address")]
    RelayBindError(#[source] io::Error),
}
