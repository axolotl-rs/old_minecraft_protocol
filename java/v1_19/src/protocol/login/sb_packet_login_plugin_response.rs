use minecraft_data::protocol::PacketContent;
use minecraft_data::protocol::PacketSwitch;
use minecraft_data::protocol::Packet;
use std::io::{BufRead, Error, ErrorKind, Result, Write};
use std::str::FromStr;

pub struct SbPacketLoginPluginResponse;

impl Packet for SbPacketLoginPluginResponse {
    type PacketIDType = i32;
    type PacketContent = PacketLoginPluginResponseContent;
    fn packet_id() -> Self::PacketIDType where Self: Sized { 2 }
}

pub struct PacketLoginPluginResponseContent {
    message_id: minecraft_data::data::VarInt,

    data: void,

}

impl PacketContent for PacketLoginPluginResponseContent {
    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<usize> {
        let mut total_bytes = 0;
        total_bytes += self.message_id.write(writer)?;
        ;

        total_bytes += self.data.write(writer)?;
        ;

        Ok(total_bytes)
    }
    fn read<Reader: BufRead>(reader: &mut Reader) -> std::io::Result<Self> {
        let message_id: minecraft_data::data::VarInt = PacketContent::read(reader)?;
        ;

        let data: void = PacketContent::read(reader)?;
        ;

        Ok(Self { message_id, data })
    }
}
