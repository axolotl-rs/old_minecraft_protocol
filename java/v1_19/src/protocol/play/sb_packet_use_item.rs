use minecraft_protocol::protocol::PacketContent;
use minecraft_protocol::protocol::PacketSwitch;
use minecraft_protocol::protocol::Packet;
use std::io::{BufRead, Error, ErrorKind, Result, Write};
use std::str::FromStr;

pub struct SbPacketUseItem;

impl Packet for SbPacketUseItem {
    type PacketIDType = i32;
    type PacketContent = PacketUseItemContent;
    fn packet_id() -> Self::PacketIDType where Self: Sized { 47 }
}

pub struct PacketUseItemContent {
    pub hand: minecraft_protocol::data::VarInt,

}

impl PacketContent for PacketUseItemContent {
    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<usize> {
        let mut total_bytes = 0;
        total_bytes += self.hand.write(writer)?;
        ;

        Ok(total_bytes)
    }
    fn read<Reader: BufRead>(reader: &mut Reader) -> std::io::Result<Self> {
        let hand: minecraft_protocol::data::VarInt = PacketContent::read(reader)?;
        ;

        Ok(Self { hand })
    }
}
