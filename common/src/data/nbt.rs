use crate::protocol::PacketContent;
use std::io::{BufRead, Write};

pub struct OptionalNbt(pub Option<axolotl_nbt::value::Value>);

impl PacketContent for OptionalNbt {
    fn read<Reader: BufRead>(reader: &mut Reader) -> std::io::Result<Self>
        where
            Self: Sized,
    {
        todo!()
    }

    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<usize>
        where
            Self: Sized,
    {
        todo!()
    }
}

pub struct Nbt(pub axolotl_nbt::value::Value);

impl PacketContent for Nbt {
    fn read<Reader: BufRead>(reader: &mut Reader) -> std::io::Result<Self>
        where
            Self: Sized,
    {
        todo!()
    }

    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<usize>
        where
            Self: Sized,
    {
        todo!()
    }
}
