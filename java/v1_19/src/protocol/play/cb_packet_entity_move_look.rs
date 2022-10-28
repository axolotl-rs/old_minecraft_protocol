use minecraft_protocol::protocol::Packet;
use minecraft_protocol::protocol::PacketContent;

use std::io::{BufRead, Write};

pub struct CbPacketEntityMoveLook;

impl Packet for CbPacketEntityMoveLook {
    type PacketIDType = i32;
    type PacketContent = PacketEntityMoveLookContent;
    fn packet_id() -> Self::PacketIDType
    where
        Self: Sized,
    {
        42
    }
}

pub struct PacketEntityMoveLookContent {
    pub entity_id: minecraft_protocol::data::VarInt,

    pub d_x: i16,

    pub d_y: i16,

    pub d_z: i16,

    pub yaw: i8,

    pub pitch: i8,

    pub on_ground: bool,
}

impl PacketContent for PacketEntityMoveLookContent {
    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<usize> {
        let mut total_bytes = 0;
        total_bytes += self.entity_id.write(writer)?;

        total_bytes += self.d_x.write(writer)?;

        total_bytes += self.d_y.write(writer)?;

        total_bytes += self.d_z.write(writer)?;

        total_bytes += self.yaw.write(writer)?;

        total_bytes += self.pitch.write(writer)?;

        total_bytes += self.on_ground.write(writer)?;

        Ok(total_bytes)
    }
    fn read<Reader: BufRead>(reader: &mut Reader) -> std::io::Result<Self> {
        let entity_id: minecraft_protocol::data::VarInt = PacketContent::read(reader)?;

        let d_x: i16 = PacketContent::read(reader)?;

        let d_y: i16 = PacketContent::read(reader)?;

        let d_z: i16 = PacketContent::read(reader)?;

        let yaw: i8 = PacketContent::read(reader)?;

        let pitch: i8 = PacketContent::read(reader)?;

        let on_ground: bool = PacketContent::read(reader)?;

        Ok(Self {
            entity_id,
            d_x,
            d_y,
            d_z,
            yaw,
            pitch,
            on_ground,
        })
    }
}
