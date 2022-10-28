use minecraft_protocol::protocol::Packet;
use minecraft_protocol::protocol::PacketContent;

use std::io::{BufRead, Write};


pub struct CbPacketCompress;
impl Packet for CbPacketCompress {
    type PacketIDType = i32;
    type PacketContent = PacketCompressContent;
    fn packet_id() -> Self::PacketIDType
    where
        Self: Sized,
    {
        3
    }
}
pub struct PacketCompressContent {
    pub threshold: minecraft_protocol::data::VarInt,
}
impl PacketContent for PacketCompressContent {
    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<usize> {
        let mut total_bytes = 0;
        total_bytes += self.threshold.write(writer)?;

        Ok(total_bytes)
    }
    fn read<Reader: BufRead>(reader: &mut Reader) -> std::io::Result<Self> {
        let threshold: minecraft_protocol::data::VarInt = PacketContent::read(reader)?;

        Ok(Self { threshold })
    }
}
