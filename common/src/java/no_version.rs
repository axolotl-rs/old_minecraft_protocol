use crate::java::handshake::HandshakePackets;
use crate::{
    ClientBoundLogin, ClientBoundPlay, ClientBoundStatus, LoginPacket, PacketHandler,
    PacketReadError, PacketReader, PacketWriteError, PacketWriter, PlayPacket, ServerBoundLogin,
    ServerBoundPlay, ServerBoundStatus, StatusPacket, Version,
};
use std::io::{BufRead, Write};

pub struct NoVersion;

impl PacketHandler for NoVersion {
    type HandleShakePacket = HandshakePackets;
    type LoginPacket = NoVersionInner;
    type StatusPacket = NoVersionInner;
    type PlayPacket = NoVersionInner;

    fn get_version() -> Version {
        Version::Java(47)
    }
}

pub struct NoVersionInner;

impl LoginPacket for NoVersionInner {
    type ServerBound = NoVersionInner;
    type ClientBound = NoVersionInner;
}

impl StatusPacket for NoVersionInner {
    type ServerBound = NoVersionInner;
    type ClientBound = NoVersionInner;
}

impl PlayPacket for NoVersionInner {
    type ServerBound = NoVersionInner;
    type ClientBound = NoVersionInner;
}

impl PacketReader for NoVersionInner {
    fn read<R: BufRead>(packet_id: i32, reader: &mut R) -> Result<Self, PacketReadError>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl ServerBoundLogin for NoVersionInner {}

impl PacketWriter for NoVersionInner {
    fn write<Writer: Write>(self, writer: &mut Writer) -> Result<usize, PacketWriteError> {
        todo!()
    }
}

impl ClientBoundLogin for NoVersionInner {}


impl From<crate::packets::play::ClientBoundPlay> for NoVersionInner {
    fn from(_: crate::packets::play::ClientBoundPlay) -> Self {
        todo!()
    }
}

impl ClientBoundPlay for NoVersionInner {}

impl Into<crate::packets::status::server_bound::ServerBoundStatus> for NoVersionInner {
    fn into(self) -> crate::packets::status::server_bound::ServerBoundStatus {
        todo!()
    }
}

impl ServerBoundStatus for NoVersionInner {}

impl From<crate::packets::status::client_bound::ClientBoundStatus> for NoVersionInner {
    fn from(_: crate::packets::status::client_bound::ClientBoundStatus) -> Self {
        todo!()
    }
}

impl ClientBoundStatus for NoVersionInner {}


impl Into<crate::packets::play::ServerBoundPlay> for NoVersionInner {
    fn into(self) -> crate::packets::play::ServerBoundPlay {
        todo!()
    }
}

impl ServerBoundPlay for NoVersionInner {}
