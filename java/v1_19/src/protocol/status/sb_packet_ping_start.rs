use minecraft_data::protocol::Packet;
use minecraft_data::protocol::PacketContent;
use minecraft_data::protocol::PacketSwitch;
use std::io::{BufRead, Error, ErrorKind, Result, Write};
use std::str::FromStr;

pub struct SbPacketPingStart;
impl Packet for SbPacketPingStart {
    type PacketIDType = i32;
    type PacketContent = PacketPingStartContent;
    fn packet_id() -> Self::PacketIDType
    where
        Self: Sized,
    {
        0
    }
}
pub struct PacketPingStartContent {}
impl PacketContent for PacketPingStartContent {
    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<usize> {
        let mut total_bytes = 0;
        Ok(total_bytes)
    }
    fn read<Reader: BufRead>(reader: &mut Reader) -> std::io::Result<Self> {
        Ok(Self {})
    }
}
