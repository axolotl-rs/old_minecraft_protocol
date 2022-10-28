use minecraft_protocol::{
    ClientBoundPlay, PacketReadError, PacketReader, PacketWriteError, PacketWriter, ServerBoundPlay,
};
use std::io::{BufRead, Write};
use minecraft_protocol::packets::play;

pub struct PlayPacket;

impl minecraft_protocol::PlayPacket for PlayPacket {
    type ServerBound = SBPlayPacket;
    type ClientBound = CBPlayPacket;
}

pub enum SBPlayPacket {

}

impl PacketReader for SBPlayPacket {
    fn read<R: BufRead>(packet_id: i32, reader: &mut R) -> Result<Self, PacketReadError>
    where
        Self: Sized,
    {
        todo!()
    }
}


impl Into<play::ServerBoundPlay> for SBPlayPacket {
    fn into(self) -> play::ServerBoundPlay {
        todo!()
    }
}

impl ServerBoundPlay for SBPlayPacket {}

pub enum CBPlayPacket {}

impl PacketWriter for CBPlayPacket {
    fn write<Writer: Write>(self, writer: &mut Writer) -> Result<usize, PacketWriteError> {
        todo!()
    }
}


impl From<play::ClientBoundPlay> for CBPlayPacket {
    fn from(_: play::ClientBoundPlay) -> Self {
        todo!()
    }
}

impl ClientBoundPlay for CBPlayPacket {}
