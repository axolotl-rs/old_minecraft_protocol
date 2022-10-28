use minecraft_protocol::protocol::Packet;
use minecraft_protocol::protocol::PacketContent;

use std::io::{BufRead, Write};

pub struct SbPacketPositionLook;
impl Packet for SbPacketPositionLook {
    type PacketIDType = i32;
    type PacketContent = PacketPositionLookContent;
    fn packet_id() -> Self::PacketIDType
    where
        Self: Sized,
    {
        18
    }
}
pub struct PacketPositionLookContent {
    pub x: f64,

    pub y: f64,

    pub z: f64,

    pub yaw: f32,

    pub pitch: f32,

    pub on_ground: bool,
}
impl PacketContent for PacketPositionLookContent {
    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<usize> {
        let mut total_bytes = 0;
        total_bytes += self.x.write(writer)?;

        total_bytes += self.y.write(writer)?;

        total_bytes += self.z.write(writer)?;

        total_bytes += self.yaw.write(writer)?;

        total_bytes += self.pitch.write(writer)?;

        total_bytes += self.on_ground.write(writer)?;

        Ok(total_bytes)
    }
    fn read<Reader: BufRead>(reader: &mut Reader) -> std::io::Result<Self> {
        let x: f64 = PacketContent::read(reader)?;

        let y: f64 = PacketContent::read(reader)?;

        let z: f64 = PacketContent::read(reader)?;

        let yaw: f32 = PacketContent::read(reader)?;

        let pitch: f32 = PacketContent::read(reader)?;

        let on_ground: bool = PacketContent::read(reader)?;

        Ok(Self {
            x,
            y,
            z,
            yaw,
            pitch,
            on_ground,
        })
    }
}
