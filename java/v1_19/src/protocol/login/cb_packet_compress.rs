use minecraft_data::protocol::Packet;
use minecraft_data::protocol::PacketContent;
use minecraft_data::protocol::PacketSwitch;
use std::io::{BufRead, Error, ErrorKind, Result, Write};
use std::str::FromStr;

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
    threshold: minecraft_data::data::VarInt,
}
impl PacketContent for PacketCompressContent {
    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<usize> {
        let mut total_bytes = 0;
        total_bytes += self.threshold.write(writer)?;

        Ok(total_bytes)
    }
    fn read<Reader: BufRead>(reader: &mut Reader) -> std::io::Result<Self> {
        let threshold: minecraft_data::data::VarInt = PacketContent::read(reader)?;

        Ok(Self { threshold })
    }
}
