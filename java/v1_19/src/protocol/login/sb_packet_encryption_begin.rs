use minecraft_data::protocol::PacketContent;
use minecraft_data::protocol::PacketSwitch;
use minecraft_data::protocol::Packet;
use std::io::{BufRead, Error, ErrorKind, Result, Write};
use std::str::FromStr;

pub struct SbPacketEncryptionBegin;

impl Packet for SbPacketEncryptionBegin {
    type PacketIDType = i32;
    type PacketContent = PacketEncryptionBeginContent;
    fn packet_id() -> Self::PacketIDType where Self: Sized { 1 }
}

pub struct PacketEncryptionBeginContent {
    shared_secret: void,

    verify_token: void,

}

impl PacketContent for PacketEncryptionBeginContent {
    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<usize> {
        let mut total_bytes = 0;
        total_bytes += self.shared_secret.write(writer)?;
        ;

        total_bytes += self.verify_token.write(writer)?;
        ;

        Ok(total_bytes)
    }
    fn read<Reader: BufRead>(reader: &mut Reader) -> std::io::Result<Self> {
        let shared_secret: void = PacketContent::read(reader)?;
        ;

        let verify_token: void = PacketContent::read(reader)?;
        ;

        Ok(Self { shared_secret, verify_token })
    }
}
