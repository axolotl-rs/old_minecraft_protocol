use minecraft_protocol::protocol::Packet;
use minecraft_protocol::protocol::PacketContent;

use std::io::{BufRead, Result, Write};

use uuid::Uuid;

pub struct CbPacketSuccess;

impl Packet for CbPacketSuccess {
    type PacketIDType = i32;
    type PacketContent = LoginSuccess;
    fn packet_id() -> Self::PacketIDType
    where
        Self: Sized,
    {
        2
    }
}

pub struct LoginSuccess {
    pub uuid: Uuid,
    pub username: String,
}

impl PacketContent for LoginSuccess {
    fn read<Reader: BufRead>(_reader: &mut Reader) -> Result<Self>
    where
        Self: Sized,
    {
        todo!()
    }

    fn write<Writer: Write>(self, writer: &mut Writer) -> Result<usize>
    where
        Self: Sized,
    {
        let mut total_bytes = 0;
        total_bytes += self.uuid.write(writer)?;
        total_bytes += self.username.write(writer)?;
        Ok(total_bytes)
    }
}

pub struct Properties {
    pub name: String,
    pub value: String,
    pub option: Option<String>,
}
