use minecraft_protocol::protocol::Packet;
use minecraft_protocol::protocol::PacketContent;

use std::io::{BufRead, Write};


pub struct CbPacketDifficulty;
impl Packet for CbPacketDifficulty {
    type PacketIDType = i32;
    type PacketContent = PacketDifficultyContent;
    fn packet_id() -> Self::PacketIDType
    where
        Self: Sized,
    {
        14
    }
}
pub struct PacketDifficultyContent {
    pub difficulty: u8,

    pub difficulty_locked: bool,
}
impl PacketContent for PacketDifficultyContent {
    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<usize> {
        let mut total_bytes = 0;
        total_bytes += self.difficulty.write(writer)?;

        total_bytes += self.difficulty_locked.write(writer)?;

        Ok(total_bytes)
    }
    fn read<Reader: BufRead>(reader: &mut Reader) -> std::io::Result<Self> {
        let difficulty: u8 = PacketContent::read(reader)?;

        let difficulty_locked: bool = PacketContent::read(reader)?;

        Ok(Self {
            difficulty,
            difficulty_locked,
        })
    }
}
