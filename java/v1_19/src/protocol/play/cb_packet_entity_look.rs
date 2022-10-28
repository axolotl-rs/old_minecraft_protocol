use minecraft_protocol::protocol::Packet;
use minecraft_protocol::protocol::PacketContent;

use std::io::{BufRead, Write};


pub struct CbPacketEntityLook;
impl Packet for CbPacketEntityLook {
    type PacketIDType = i32;
    type PacketContent = PacketEntityLookContent;
    fn packet_id() -> Self::PacketIDType
    where
        Self: Sized,
    {
        43
    }
}
pub struct PacketEntityLookContent {
    pub entity_id: minecraft_protocol::data::VarInt,

    pub yaw: i8,

    pub pitch: i8,

    pub on_ground: bool,
}
impl PacketContent for PacketEntityLookContent {
    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<usize> {
        let mut total_bytes = 0;
        total_bytes += self.entity_id.write(writer)?;

        total_bytes += self.yaw.write(writer)?;

        total_bytes += self.pitch.write(writer)?;

        total_bytes += self.on_ground.write(writer)?;

        Ok(total_bytes)
    }
    fn read<Reader: BufRead>(reader: &mut Reader) -> std::io::Result<Self> {
        let entity_id: minecraft_protocol::data::VarInt = PacketContent::read(reader)?;

        let yaw: i8 = PacketContent::read(reader)?;

        let pitch: i8 = PacketContent::read(reader)?;

        let on_ground: bool = PacketContent::read(reader)?;

        Ok(Self {
            entity_id,
            yaw,
            pitch,
            on_ground,
        })
    }
}
