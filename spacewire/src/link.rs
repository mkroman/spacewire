use std::convert::TryInto;

use log::{debug, trace};
use tokio::io::{AsyncWriteExt, BufWriter};
use tokio::net::{self, TcpStream, ToSocketAddrs};

use crate::proto::{Packet, PROTOCOL_VERSION};
use crate::Error;
use crate::Identity;

#[derive(Debug)]
pub struct Link {
    relay: String,
    identity: Identity,
    msg_id: u64,
}

/// Resolves the given `host` and attempts to open a TCP connection to it on the given `port`.
async fn resolve_and_connect(addr: impl ToSocketAddrs) -> Result<Option<TcpStream>, Error> {
    // Resolve the address and find one that is connectable
    let addrs = net::lookup_host(addr).await?;

    for addr in addrs {
        if let Ok(stream) = TcpStream::connect(addr).await {
            trace!("Connected to relay tcp://{}", stream.peer_addr()?);

            return Ok(Some(stream));
        }
    }

    Ok(None)
}

impl Link {
    /// Constructs a new [`Link`] that connects to the given `relay` using the given `identity` for
    /// encryption.
    pub fn new_with_identity(relay: impl Into<String>, identity: Identity) -> Result<Link, Error> {
        Ok(Link {
            relay: relay.into(),
            identity,
            msg_id: 0,
        })
    }

    /// Attempts to the previously defined `relay`.
    pub async fn connect(&self) -> Result<(), Error> {
        if let Some(stream) = resolve_and_connect(&self.relay).await? {
            debug!(
                "Established connection to relay {}",
                stream.peer_addr().unwrap()
            );

            let mut writer = BufWriter::new(stream);

            // Introduce ourself
            let packet = Packet::ClientIntroduction {
                proto_ver: PROTOCOL_VERSION,
                pub_key: self.identity.public_key().try_into().unwrap(),
            };

            packet.to_writer(&mut writer).await?;
            writer.flush().await?;
        }

        Ok(())
    }

    /// Returns the associated `identity`.
    pub fn identity(&self) -> &Identity {
        &self.identity
    }
}
