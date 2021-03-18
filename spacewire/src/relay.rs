use bytes::{Buf, BytesMut};
use log::{debug, error, trace};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};

use std::convert::TryInto;
use std::io::Cursor;

use crate::proto::{Packet, PacketError, PacketType};
use crate::Error;

const RECV_BUFFER_SIZE: usize = 4096;
/// The minimum packet length is 10 bytes (id as u16 + len as u64)
const MIN_PACKET_LEN: usize = 10;

pub struct Relay {
    listener: TcpListener,
}

async fn handshake(socket: TcpStream) -> Result<(), Error> {
    let mut conn = Connection::new(socket);

    match conn.read_packet().await? {
        Some(Packet::ClientIntroduction { proto_ver, pub_key }) => {
            debug!(
                "Opened connection using protocol version {} with client {:?}",
                proto_ver, pub_key
            );

            Ok(())
        }
        _ => {
            debug!("Received unimplemented packet - closing connection");

            conn.close().await?;

            return Err(Error::UnexpectedPacket);
        }
    }
}

struct Connection<S: AsyncWrite + AsyncWriteExt + AsyncRead + Unpin> {
    stream: S,
    buffer: BytesMut,
}

impl<S: AsyncWrite + AsyncWriteExt + AsyncRead + Unpin> Connection<S> {
    pub fn new(stream: S) -> Self {
        Connection {
            stream,
            buffer: BytesMut::with_capacity(RECV_BUFFER_SIZE),
        }
    }

    pub async fn close(&mut self) -> Result<(), Error> {
        self.stream.shutdown().await.map_err(Error::IoError)
    }

    pub async fn read_packet(&mut self) -> Result<Option<Packet>, Error> {
        loop {
            if let Some(packet) = self.parse_packet()? {
                return Ok(Some(packet));
            }

            // There is not enough buffered data to read a frame.
            // Attempt to read more data from the socket.
            //
            // On success, the number of bytes is returned. `0`
            // indicates "end of stream".
            if 0 == self.stream.read_buf(&mut self.buffer).await? {
                // The remote closed the connection. For this to be
                // a clean shutdown, there should be no data in the
                // read buffer. If there is, this means that the
                // peer closed the socket while sending a frame.
                if self.buffer.is_empty() {
                    return Ok(None);
                } else {
                    return Err(Error::ConnectionReset);
                }
            }
        }
    }

    /// Attempts to parse a packet from the current data in the buffer.
    ///
    /// Returns `Ok(Some(packet))` on success, `Ok(None)` when there's no packet yet and `Err(err)`
    /// on error.
    pub fn parse_packet(&mut self) -> Result<Option<Packet>, Error> {
        if self.buffer.len() < MIN_PACKET_LEN {
            return Ok(None);
        }

        let mut buf = Cursor::new(&self.buffer[..]);

        let typ = buf
            .get_u16()
            .try_into()
            .map_err(|_| PacketError::InvalidPacketId)?;
        let len = buf.get_u64();

        // Wait for the rest of the packet
        if (self.buffer.len() as u64) < len {
            return Ok(None);
        }

        match typ {
            PacketType::ClientIntroduction => {
                if self.buffer.len() > 34 {
                    let proto_ver = buf.get_u16();
                    let pub_key = buf.chunk()[..32].try_into().unwrap();

                    debug!("proto ver: {} pub key: {:?}", proto_ver, pub_key);

                    let pos = buf.position() as usize;

                    self.buffer.advance(pos);

                    return Ok(Some(Packet::ClientIntroduction { proto_ver, pub_key }));
                }
            }
        }

        Ok(None)
    }
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
                Ok((socket, addr)) => {
                    debug!("Accepted new connection from {}", addr);

                    if let Err(e) = handshake(socket).await {
                        debug!(
                            "Connection {} closed while waiting for handshake: {}",
                            addr, e
                        );
                    }
                }
                Err(e) => error!("Could not accept connection: {}", e),
            }
        }
    }
}
