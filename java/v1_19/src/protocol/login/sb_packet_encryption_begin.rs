use crate::protocol::login::SigData;

use minecraft_protocol::protocol::Packet;
use minecraft_protocol::protocol::PacketContent;

use std::io::{BufRead, Write};

pub struct SbPacketEncryptionBegin;

impl Packet for SbPacketEncryptionBegin {
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
    pub sig: SigData, // Currently reusing the same struct as the login packet. .
}

impl PacketContent for PacketEncryptionBeginContent {
    fn read<Reader: BufRead>(reader: &mut Reader) -> std::io::Result<Self> {
        Ok(Self {
            sig: SigData::read(reader)?,
        })
    }
    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<usize> {
        let mut total_bytes = 0;
        total_bytes += self.sig.write(writer)?;

        Ok(total_bytes)
    }
}
