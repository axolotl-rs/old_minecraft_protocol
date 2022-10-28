use minecraft_protocol::protocol::Packet;
use minecraft_protocol::protocol::PacketContent;

use std::io::{BufRead, Write};

pub struct CbPacketDisconnect;
impl Packet for CbPacketDisconnect {
    type PacketIDType = i32;
    type PacketContent = PacketDisconnectContent;
    fn packet_id() -> Self::PacketIDType
    where
        Self: Sized,
    {
        0
    }
}
pub struct PacketDisconnectContent {
    pub reason: String,
}
impl PacketContent for PacketDisconnectContent {
    fn read<Reader: BufRead>(reader: &mut Reader) -> std::io::Result<Self> {
        let reason: String = PacketContent::read(reader)?;

        Ok(Self { reason })
    }
    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<usize> {
        let mut total_bytes = 0;
        total_bytes += self.reason.write(writer)?;

        Ok(total_bytes)
    }
}
