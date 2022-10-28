use minecraft_protocol::protocol::Packet;
use minecraft_protocol::protocol::PacketContent;

use std::io::{BufRead, Write};

pub struct CbPacketEntityHeadRotation;
impl Packet for CbPacketEntityHeadRotation {
    type PacketIDType = i32;
    type PacketContent = PacketEntityHeadRotationContent;
    fn packet_id() -> Self::PacketIDType
    where
        Self: Sized,
    {
        62
    }
}
pub struct PacketEntityHeadRotationContent {
    pub entity_id: minecraft_protocol::data::VarInt,

    pub head_yaw: i8,
}
impl PacketContent for PacketEntityHeadRotationContent {
    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<usize> {
        let mut total_bytes = 0;
        total_bytes += self.entity_id.write(writer)?;

        total_bytes += self.head_yaw.write(writer)?;

        Ok(total_bytes)
    }
    fn read<Reader: BufRead>(reader: &mut Reader) -> std::io::Result<Self> {
        let entity_id: minecraft_protocol::data::VarInt = PacketContent::read(reader)?;

        let head_yaw: i8 = PacketContent::read(reader)?;

        Ok(Self {
            entity_id,
            head_yaw,
        })
    }
}
