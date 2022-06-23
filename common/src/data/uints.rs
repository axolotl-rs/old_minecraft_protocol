use crate::protocol::PacketContent;
use byteorder::{ReadBytesExt, WriteBytesExt};
use std::io::{BufRead, Write};

impl PacketContent for u8 {
    fn read<R: BufRead>(buf: &mut R) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        buf.read_u8()
    }

    fn write<W: Write>(self, write: &mut W) -> std::io::Result<usize>
    where
        Self: Sized,
    {
        write.write_u8(self)?;
        Ok(1)
    }
}

impl PacketContent for u16 {
    fn read<R: BufRead>(buf: &mut R) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        buf.read_u16::<byteorder::BigEndian>()
    }

    fn write<W: Write>(self, write: &mut W) -> std::io::Result<usize>
    where
        Self: Sized,
    {
        write.write_u16::<byteorder::BigEndian>(self)?;
        Ok(2)
    }
}

impl PacketContent for u32 {
    fn read<R: BufRead>(buf: &mut R) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        buf.read_u32::<byteorder::BigEndian>()
    }

    fn write<W: Write>(self, write: &mut W) -> std::io::Result<usize>
    where
        Self: Sized,
    {
        write.write_u32::<byteorder::BigEndian>(self)?;
        Ok(4)
    }
}

impl PacketContent for u64 {
    fn read<R: BufRead>(buf: &mut R) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        buf.read_u64::<byteorder::BigEndian>()
    }

    fn write<W: Write>(self, write: &mut W) -> std::io::Result<usize>
    where
        Self: Sized,
    {
        write.write_u64::<byteorder::BigEndian>(self)?;
        Ok(8)
    }
}
