use minecraft_protocol::protocol::Packet;
use minecraft_protocol::protocol::PacketContent;

use std::io::{BufRead, Write};

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
    fn write<Writer: Write>(self, _writer: &mut Writer) -> std::io::Result<usize> {
        let total_bytes = 0;
        Ok(total_bytes)
    }
    fn read<Reader: BufRead>(_reader: &mut Reader) -> std::io::Result<Self> {
        Ok(Self {})
    }
}
