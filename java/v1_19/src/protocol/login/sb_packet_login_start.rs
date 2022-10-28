use crate::protocol::login::SigData;


use minecraft_protocol::protocol::Packet;
use minecraft_protocol::protocol::PacketContent;

use std::io::{BufRead, Write};


pub struct SbPacketLoginStart;

impl Packet for SbPacketLoginStart {
    type PacketIDType = i32;
    type PacketContent = PacketLoginStartContent;
    fn packet_id() -> Self::PacketIDType
    where
        Self: Sized,
    {
        0
    }
}

pub struct PacketLoginStartContent {
    pub username: String,
    pub has_sig_data: bool,
    pub timestamp: Option<i64>,
    pub sig_data: Option<SigData>,
}

impl PacketContent for PacketLoginStartContent {
    fn read<Reader: BufRead>(reader: &mut Reader) -> std::io::Result<Self> {
        let username: String = PacketContent::read(reader)?;
        let (has_sig_data, timestamp, sig_data) = if bool::read(reader)? {
            (true, Some(i64::read(reader)?), Some(SigData::read(reader)?))
        } else {
            (false, None, None)
        };

        Ok(Self {
            username,
            has_sig_data,
            timestamp,
            sig_data,
        })
    }
    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<usize> {
        let mut total_bytes = 0;
        total_bytes += self.username.write(writer)?;

        Ok(total_bytes)
    }
}
