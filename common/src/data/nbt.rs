use crate::protocol::PacketContent;
use std::io::{BufRead, Write};

#[derive(Debug, Clone, Default)]
pub struct OptionalNbt(pub Option<axolotl_nbt::value::Value>);

impl PacketContent for OptionalNbt {
    fn read<Reader: BufRead>(_reader: &mut Reader) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        todo!()
    }

    fn write<Writer: Write>(self, _writer: &mut Writer) -> std::io::Result<usize>
    where
        Self: Sized,
    {
        todo!()
    }
}

pub struct Nbt(pub axolotl_nbt::value::Value);

impl PacketContent for Nbt {
    fn read<Reader: BufRead>(_reader: &mut Reader) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        todo!()
    }

    fn write<Writer: Write>(self, _writer: &mut Writer) -> std::io::Result<usize>
    where
        Self: Sized,
    {
        todo!()
    }
}
