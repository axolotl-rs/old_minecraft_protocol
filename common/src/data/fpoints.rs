use crate::protocol::PacketContent;
use byteorder::{ReadBytesExt, WriteBytesExt};
use std::io::{BufRead, Write};

impl PacketContent for f32 {
    fn read<Reader: BufRead>(reader: &mut Reader) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        reader.read_f32::<byteorder::BigEndian>()
    }

    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<usize>
    where
        Self: Sized,
    {
        writer.write_f32::<byteorder::BigEndian>(self)?;
        Ok(4)
    }
}

impl PacketContent for f64 {
    fn read<Reader: BufRead>(reader: &mut Reader) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        reader.read_f64::<byteorder::BigEndian>()
    }

    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<usize>
    where
        Self: Sized,
    {
        writer.write_f64::<byteorder::BigEndian>(self)?;
        Ok(8)
    }
}
