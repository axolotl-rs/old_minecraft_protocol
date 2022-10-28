use minecraft_protocol::protocol::Packet;
use minecraft_protocol::protocol::PacketContent;

use std::io::{BufRead, Write};


pub struct CbPacketPosition;
impl Packet for CbPacketPosition {
    type PacketIDType = i32;
    type PacketContent = PacketPositionContent;
    fn packet_id() -> Self::PacketIDType
    where
        Self: Sized,
    {
        56
    }
}
pub struct PacketPositionContent {
    pub x: f64,

    pub y: f64,

    pub z: f64,

    pub yaw: f32,

    pub pitch: f32,

    pub flags: i8,

    pub teleport_id: minecraft_protocol::data::VarInt,

    pub dismount_vehicle: bool,
}
impl PacketContent for PacketPositionContent {
    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<usize> {
        let mut total_bytes = 0;
        total_bytes += self.x.write(writer)?;

        total_bytes += self.y.write(writer)?;

        total_bytes += self.z.write(writer)?;

        total_bytes += self.yaw.write(writer)?;

        total_bytes += self.pitch.write(writer)?;

        total_bytes += self.flags.write(writer)?;

        total_bytes += self.teleport_id.write(writer)?;

        total_bytes += self.dismount_vehicle.write(writer)?;

        Ok(total_bytes)
    }
    fn read<Reader: BufRead>(reader: &mut Reader) -> std::io::Result<Self> {
        let x: f64 = PacketContent::read(reader)?;

        let y: f64 = PacketContent::read(reader)?;

        let z: f64 = PacketContent::read(reader)?;

        let yaw: f32 = PacketContent::read(reader)?;

        let pitch: f32 = PacketContent::read(reader)?;

        let flags: i8 = PacketContent::read(reader)?;

        let teleport_id: minecraft_protocol::data::VarInt = PacketContent::read(reader)?;

        let dismount_vehicle: bool = PacketContent::read(reader)?;

        Ok(Self {
            x,
            y,
            z,
            yaw,
            pitch,
            flags,
            teleport_id,
            dismount_vehicle,
        })
    }
}
