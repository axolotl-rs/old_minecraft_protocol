use std::io::{BufRead, Write};
use crate::common::protocol::PacketContent;

pub struct OptionalNbt{

}
impl PacketContent for OptionalNbt{
    fn read<Reader: BufRead>(reader: &mut Reader) -> std::io::Result<Self> where Self: Sized {
        todo!()
    }

    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<usize> where Self: Sized {
        todo!()
    }
}
pub struct Nbt{

}
impl PacketContent for Nbt{
    fn read<Reader: BufRead>(reader: &mut Reader) -> std::io::Result<Self> where Self: Sized {
        todo!()
    }

    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<usize> where Self: Sized {
        todo!()
    }
}