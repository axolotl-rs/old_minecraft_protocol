use crate::protocol::PacketContent;
use byteorder::{ReadBytesExt, WriteBytesExt};
use std::io::{BufRead, Write};
impl PacketContent for i8 {
    fn read<Reader: BufRead>(reader: &mut Reader) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        reader.read_i8()
    }

    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<usize>
    where
        Self: Sized,
    {
        writer.write_i8(self)?;
        Ok(1)
    }
}
impl PacketContent for i16 {
    fn read<Reader: BufRead>(reader: &mut Reader) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        reader.read_i16::<byteorder::BigEndian>()
    }

    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<usize>
    where
        Self: Sized,
    {
        writer.write_i16::<byteorder::BigEndian>(self)?;
        Ok(2)
    }
}
impl PacketContent for i32 {
    fn read<Reader: BufRead>(reader: &mut Reader) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        reader.read_i32::<byteorder::BigEndian>()
    }

    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<usize>
    where
        Self: Sized,
    {
        writer.write_i32::<byteorder::BigEndian>(self)?;
        Ok(4)
    }
}
impl PacketContent for i64 {
    fn read<Reader: BufRead>(reader: &mut Reader) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        reader.read_i64::<byteorder::BigEndian>()
    }

    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<usize>
    where
        Self: Sized,
    {
        writer.write_i64::<byteorder::BigEndian>(self)?;
        Ok(8)
    }
}
