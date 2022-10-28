use minecraft_protocol::protocol::Packet;
use minecraft_protocol::protocol::PacketContent;

use std::io::{BufRead, Write};

pub struct CbPacketEntityVelocity;
impl Packet for CbPacketEntityVelocity {
    type PacketIDType = i32;
    type PacketContent = PacketEntityVelocityContent;
    fn packet_id() -> Self::PacketIDType
    where
        Self: Sized,
    {
        79
    }
}
pub struct PacketEntityVelocityContent {
    pub entity_id: minecraft_protocol::data::VarInt,

    pub velocity_x: i16,

    pub velocity_y: i16,

    pub velocity_z: i16,
}
impl PacketContent for PacketEntityVelocityContent {
    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<usize> {
        let mut total_bytes = 0;
        total_bytes += self.entity_id.write(writer)?;

        total_bytes += self.velocity_x.write(writer)?;

        total_bytes += self.velocity_y.write(writer)?;

        total_bytes += self.velocity_z.write(writer)?;

        Ok(total_bytes)
    }
    fn read<Reader: BufRead>(reader: &mut Reader) -> std::io::Result<Self> {
        let entity_id: minecraft_protocol::data::VarInt = PacketContent::read(reader)?;

        let velocity_x: i16 = PacketContent::read(reader)?;

        let velocity_y: i16 = PacketContent::read(reader)?;

        let velocity_z: i16 = PacketContent::read(reader)?;

        Ok(Self {
            entity_id,
            velocity_x,
            velocity_y,
            velocity_z,
        })
    }
}
