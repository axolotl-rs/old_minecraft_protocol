use minecraft_data::protocol::Packet;
use minecraft_data::protocol::PacketContent;
use minecraft_data::protocol::PacketSwitch;
use std::io::{BufRead, Error, ErrorKind, Result, Write};
use std::str::FromStr;

pub struct SbPacketLoginStart;
impl Packet for SbPacketLoginStart {
    type PacketIDType = i32;
    type PacketContent = PacketLoginStartContent;
    fn packet_id() -> Self::PacketIDType
    where
        Self: Sized,
    {
        0
    }
}
pub struct PacketLoginStartContent {
    pub username: String,
}
impl PacketContent for PacketLoginStartContent {
    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<usize> {
        let mut total_bytes = 0;
        total_bytes += self.username.write(writer)?;

        Ok(total_bytes)
    }
    fn read<Reader: BufRead>(reader: &mut Reader) -> std::io::Result<Self> {
        let username: String = PacketContent::read(reader)?;

        Ok(Self { username })
    }
}
