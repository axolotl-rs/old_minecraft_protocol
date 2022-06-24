use minecraft_data::protocol::Packet;
use minecraft_data::protocol::PacketContent;
use minecraft_data::protocol::PacketSwitch;
use std::io::{BufRead, Error, ErrorKind, Result, Write};
use std::str::FromStr;
use bytes::Bytes;
use crate::protocol::login::SigData;

pub struct CbPacketEncryptionBegin;

impl Packet for CbPacketEncryptionBegin {
    type PacketIDType = i32;
    type PacketContent = PacketEncryptionBeginContent;
    fn packet_id() -> Self::PacketIDType
        where
            Self: Sized,
    {
        1
    }
}

pub struct PacketEncryptionBeginContent {
    pub server_id: String,
    pub sig: SigData, // ReUsing the same struct as the login packet.
}

impl PacketContent for PacketEncryptionBeginContent {
    fn read<Reader: BufRead>(reader: &mut Reader) -> std::io::Result<Self> {
        let server_id: String = PacketContent::read(reader)?;

        let sig: SigData = PacketContent::read(reader)?;


        Ok(Self {
            server_id,
            sig,
        })
    }
    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<usize> {
        let mut total_bytes = 0;
        total_bytes += self.server_id.write(writer)?;
        total_bytes += self.sig.write(writer)?;


        Ok(total_bytes)
    }
}
