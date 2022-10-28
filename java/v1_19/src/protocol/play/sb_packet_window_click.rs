use minecraft_protocol::protocol::Packet;
use minecraft_protocol::protocol::PacketContent;
use minecraft_protocol::protocol::PacketSwitch;
use std::io::{BufRead, Error, ErrorKind, Result, Write};
use std::str::FromStr;

pub struct SbPacketWindowClick;
impl Packet for SbPacketWindowClick {
    type PacketIDType = i32;
    type PacketContent = PacketWindowClickContent;
    fn packet_id() -> Self::PacketIDType
    where
        Self: Sized,
    {
        8
    }
}
pub struct PacketWindowClickContent {
    pub window_id: u8,

    pub state_id: minecraft_protocol::data::VarInt,

    pub slot: i16,

    pub mouse_button: i8,

    pub mode: minecraft_protocol::data::VarInt,

    pub changed_slots: PacketWindowClickContentArray,

    pub cursor_item: crate::protocol::types::slot::Slot,
}
impl PacketContent for PacketWindowClickContent {
    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<usize> {
        let mut total_bytes = 0;
        total_bytes += self.window_id.write(writer)?;

        total_bytes += self.state_id.write(writer)?;

        total_bytes += self.slot.write(writer)?;

        total_bytes += self.mouse_button.write(writer)?;

        total_bytes += self.mode.write(writer)?;

        total_bytes += self.changed_slots.write(writer)?;

        total_bytes += self.cursor_item.write(writer)?;

        Ok(total_bytes)
    }
    fn read<Reader: BufRead>(reader: &mut Reader) -> std::io::Result<Self> {
        let window_id: u8 = PacketContent::read(reader)?;

        let state_id: minecraft_protocol::data::VarInt = PacketContent::read(reader)?;

        let slot: i16 = PacketContent::read(reader)?;

        let mouse_button: i8 = PacketContent::read(reader)?;

        let mode: minecraft_protocol::data::VarInt = PacketContent::read(reader)?;

        let changed_slots: PacketWindowClickContentArray = PacketContent::read(reader)?;

        let cursor_item: crate::protocol::types::slot::Slot = PacketContent::read(reader)?;

        Ok(Self {
            window_id,
            state_id,
            slot,
            mouse_button,
            mode,
            changed_slots,
            cursor_item,
        })
    }
}
pub type PacketWindowClickContentArray = Vec<PacketWindowClickContentArrayContent>;
pub struct PacketWindowClickContentArrayContent {
    pub location: i16,

    pub item: crate::protocol::types::slot::Slot,
}
impl PacketContent for PacketWindowClickContentArrayContent {
    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<usize> {
        let mut total_bytes = 0;
        total_bytes += self.location.write(writer)?;

        total_bytes += self.item.write(writer)?;

        Ok(total_bytes)
    }
    fn read<Reader: BufRead>(reader: &mut Reader) -> std::io::Result<Self> {
        let location: i16 = PacketContent::read(reader)?;

        let item: crate::protocol::types::slot::Slot = PacketContent::read(reader)?;

        Ok(Self { location, item })
    }
}
