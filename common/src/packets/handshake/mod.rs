use crate::data::VarInt;
use crate::protocol::{Packet, PacketContent};
use std::io::{BufRead, Write};

pub struct SetProtocol;

impl Packet for SetProtocol {
    type PacketIDType = i32;
    type PacketContent = PacketSetProtocolContent;
    fn packet_id() -> Self::PacketIDType
        where
            Self: Sized,
    {
        0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacketSetProtocolContent {
    pub protocol_version: VarInt,
    pub server_host: String,
    pub server_port: u16,
    pub next_state: VarInt,
}

impl PacketContent for PacketSetProtocolContent {
    fn read<Reader: BufRead>(reader: &mut Reader) -> std::io::Result<Self> {
        let protocol_version: VarInt = PacketContent::read(reader)?;

        let server_host: String = PacketContent::read(reader)?;

        let server_port: u16 = PacketContent::read(reader)?;

        let next_state: VarInt = PacketContent::read(reader)?;

        Ok(Self {
            protocol_version,
            server_host,
            server_port,
            next_state,
        })
    }
    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<usize> {
        let mut total_bytes = 0;
        total_bytes += self.protocol_version.write(writer)?;

        total_bytes += self.server_host.write(writer)?;

        total_bytes += self.server_port.write(writer)?;

        total_bytes += self.next_state.write(writer)?;

        Ok(total_bytes)
    }
}

pub struct SbPacketLegacyServerListPing;

impl Packet for SbPacketLegacyServerListPing {
    type PacketIDType = i32;
    type PacketContent = PacketLegacyServerListPingContent;
    fn packet_id() -> Self::PacketIDType
        where
            Self: Sized,
    {
        254
    }
}

#[derive(Debug)]
pub struct PacketLegacyServerListPingContent {
    payload: u8,
}

impl PacketContent for PacketLegacyServerListPingContent {
    fn read<Reader: BufRead>(reader: &mut Reader) -> std::io::Result<Self> {
        let payload: u8 = PacketContent::read(reader)?;

        Ok(Self { payload })
    }
    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<usize> {
        let mut total_bytes = 0;
        total_bytes += self.payload.write(writer)?;

        Ok(total_bytes)
    }
}
