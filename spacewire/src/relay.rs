use log::{debug, error, trace};
use tokio::net::{TcpListener, ToSocketAddrs};

use crate::Error;

pub struct Relay {
    listener: TcpListener,
}

impl Relay {
    /// Attempts to bind a relay to the first resolving `addr`, returning a [`Relay`] on success.
    pub async fn bind<Addr: ToSocketAddrs>(addr: Addr) -> Result<Relay, Error> {
        let addr = tokio::net::lookup_host(addr)
            .await
            .map_err(Error::BindAddressResolveError)?
            .next()
            .ok_or(Error::InvalidBindAddress)?;

        trace!("Binding new relay to tcp://{}", addr);

        let listener = TcpListener::bind(addr)
            .await
            .map_err(Error::RelayBindError)?;

        trace!("Relay bound to tcp://{}", listener.local_addr()?);

        Ok(Relay { listener })
    }

    /// Continually polls the relay instance for new connections in a blocking fashion.
    pub async fn poll(&self) -> Result<(), Error> {
        loop {
            match self.listener.accept().await {
                Ok((_socket, addr)) => {
                    debug!("Accepted new connection from {}", addr);
                }
                Err(e) => error!("Could not accept connection: {}", e),
            }
        }
    }
}
