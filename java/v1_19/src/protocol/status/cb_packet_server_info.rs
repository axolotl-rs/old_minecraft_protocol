use minecraft_protocol::protocol::Packet;
use minecraft_protocol::protocol::PacketContent;

use std::io::{BufRead, Write};

pub struct CbPacketServerInfo;
impl Packet for CbPacketServerInfo {
    type PacketIDType = i32;
    type PacketContent = PacketServerInfoContent;
    fn packet_id() -> Self::PacketIDType
    where
        Self: Sized,
    {
        0
    }
}
pub struct PacketServerInfoContent {
    pub response: String,
}
impl PacketContent for PacketServerInfoContent {
    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<usize> {
        let mut total_bytes = 0;
        total_bytes += self.response.write(writer)?;

        Ok(total_bytes)
    }
    fn read<Reader: BufRead>(reader: &mut Reader) -> std::io::Result<Self> {
        let response: String = PacketContent::read(reader)?;

        Ok(Self { response })
    }
}
