//! Protocol implementation

use num_enum::{IntoPrimitive, TryFromPrimitive};
use thiserror::Error;
use tokio::io::{self, AsyncWrite, AsyncWriteExt};

/// Protocol types and their IDs:
///
/// 0 (SERVER_INTRODUCTION):
/// The first message that the server sends to the client, which includes the servers identity
/// and the protocol version.

/// The current protocol version.
///
/// This is currently 0 because the protocol is still under active development.
pub const PROTOCOL_VERSION: u16 = 0;

#[repr(u16)]
#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum PacketType {
    /// CLIENT_INTRODUCTION - (1):
    ///
    /// The first message that the client sends to the server upon connection, which includes the
    /// clients identity and the protocol version.
    ///
    /// ### Fields
    ///
    /// | Offset | Size (bytes) | Field            | Description                     |
    /// |--------|--------------|------------------|---------------------------------|
    /// | 0x00   | 0x002        | protocol_version | The current protocol version    |
    /// | 0x02   | 0x100        | identity_pub_key | The clients public identity key |
    ClientIntroduction = 0,
}

#[derive(Error, Debug)]
pub enum PacketError {
    #[error("Invalid packet id")]
    InvalidPacketId,
    #[error("Incomplete packet")]
    Incomplete,
}

#[derive(Debug)]
pub enum Packet {
    ClientIntroduction { proto_ver: u16, pub_key: [u8; 32] },
}

impl Packet {
    pub async fn to_writer<W: Unpin + AsyncWrite>(&self, mut buf: W) -> Result<(), io::Error> {
        match *self {
            Packet::ClientIntroduction { proto_ver, pub_key } => {
                buf.write_u16(PacketType::ClientIntroduction.into()).await?;
                buf.write_u64(32 + 2).await?;
                buf.write_u16(proto_ver).await?;
                buf.write_all(&pub_key).await?;
            }
        }

        Ok(())
    }
}
