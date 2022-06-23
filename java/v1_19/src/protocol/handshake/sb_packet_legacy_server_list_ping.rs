use minecraft_data::protocol::Packet;
use minecraft_data::protocol::PacketContent;
use minecraft_data::protocol::PacketSwitch;
use std::io::{BufRead, Error, ErrorKind, Result, Write};
use std::str::FromStr;

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
pub struct PacketLegacyServerListPingContent {
    payload: u8,
}
impl PacketContent for PacketLegacyServerListPingContent {
    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<usize> {
        let mut total_bytes = 0;
        total_bytes += self.payload.write(writer)?;

        Ok(total_bytes)
    }
    fn read<Reader: BufRead>(reader: &mut Reader) -> std::io::Result<Self> {
        let payload: u8 = PacketContent::read(reader)?;

        Ok(Self { payload })
    }
}
