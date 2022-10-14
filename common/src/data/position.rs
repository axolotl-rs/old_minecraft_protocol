use crate::protocol::PacketContent;
use std::io::{BufRead, Write};

pub struct Position {}
impl PacketContent for Position {
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
