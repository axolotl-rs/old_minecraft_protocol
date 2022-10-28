pub mod data;
pub mod packets;

/// The common version types
pub mod java;
pub mod protocol;

use std::io::{BufRead, Write};

pub trait Variation {
    type Metadata;
    type DisplayName;

    fn metadata(&self) -> Self::Metadata;

    fn display_name(&self) -> Self::DisplayName;
}

#[test]
pub fn test() {}

/// A Write Error for a Packet
#[derive(Debug, thiserror::Error)]
pub enum PacketWriteError {
    /// IO Error
    #[error("Failed to write value: {0}")]
    IOError(std::io::Error),
    /// Any internal error
    #[error("Failed to write value: {0}")]
    ContentError(Box<dyn std::error::Error + Send + Sync + 'static>),
}

impl From<std::io::Error> for PacketWriteError {
    fn from(value: std::io::Error) -> Self {
        PacketWriteError::IOError(value)
    }
}

/// A Read Error for a Packet
#[derive(Debug, thiserror::Error)]
pub enum PacketReadError {
    /// IO Error
    #[error("Failed to write value: {0}")]
    IOError(std::io::Error),
    /// Could be any internal Error
    #[error("Failed to write value: {0}")]
    ContentError(Box<dyn std::error::Error + Send + Sync + 'static>),
    #[error("Failed to write value: {0}")]
    MalformedPacket(&'static str),
}

impl From<std::io::Error> for PacketReadError {
    fn from(value: std::io::Error) -> Self {
        PacketReadError::IOError(value)
    }
}

#[derive(Debug)]
pub enum Version {
    Java(i32),
}

pub trait PacketReader {
    fn read<R: BufRead>(packet_id: i32, reader: &mut R) -> Result<Self, PacketReadError>
        where
            Self: Sized;
}

pub trait PacketWriter {
    fn write<Writer: Write>(self, writer: &mut Writer) -> Result<usize, PacketWriteError>;
}

pub trait PacketHandler {
    type HandleShakePacket: HandshakePacket;
    type LoginPacket: LoginPacket;
    type StatusPacket: StatusPacket;
    type PlayPacket: PlayPacket;

    fn get_version() -> Version;
}

pub trait HandshakePacket {
    type ServerBound: ServerBoundHandShake;
    type ClientBound;
}

pub trait ServerBoundHandShake: PacketReader {}

pub trait LoginPacket {
    type ServerBound: ServerBoundLogin;
    type ClientBound: ClientBoundLogin;
}

pub trait ServerBoundLogin: PacketReader {}

pub trait ClientBoundLogin: PacketWriter {}

pub trait PlayPacket {
    type ServerBound: ServerBoundPlay;
    type ClientBound: ClientBoundPlay;
}

pub trait ServerBoundPlay: PacketReader + Into<packets::play::ServerBoundPlay> {}

pub trait ClientBoundPlay: PacketWriter + From<packets::play::ClientBoundPlay> {}

pub trait StatusPacket {
    type ServerBound: ServerBoundStatus;
    type ClientBound: ClientBoundStatus;
}

pub trait ServerBoundStatus: PacketReader + Into<packets::status::server_bound::ServerBoundStatus> {}

pub trait ClientBoundStatus: PacketWriter + From<packets::status::client_bound::ClientBoundStatus> {}
