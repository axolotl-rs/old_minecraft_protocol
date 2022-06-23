use minecraft_data::protocol::Packet;
use minecraft_data::protocol::PacketContent;
use minecraft_data::protocol::PacketSwitch;
use std::io::{BufRead, Error, ErrorKind, Result, Write};
use std::str::FromStr;

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
    reason: String,
}
impl PacketContent for PacketDisconnectContent {
    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<usize> {
        let mut total_bytes = 0;
        total_bytes += self.reason.write(writer)?;

        Ok(total_bytes)
    }
    fn read<Reader: BufRead>(reader: &mut Reader) -> std::io::Result<Self> {
        let reason: String = PacketContent::read(reader)?;

        Ok(Self { reason })
    }
}
