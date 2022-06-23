pub mod bitfield;
pub mod fpoints;
pub mod nbt;
pub mod position;
pub mod sints;
pub mod uints;
mod var_int;

use crate::protocol::PacketContent;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use bytes::{BufMut, BytesMut};
use std::io::{BufRead, Read, Write};
use uuid::{Bytes, Uuid};
pub use var_int::VarInt;

pub struct Void;

impl PacketContent for Void {
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

impl PacketContent for bool {
    fn read<R: BufRead>(buf: &mut R) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        let i = buf.read_u8()?;
        Ok(i != 0)
    }

    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<usize>
    where
        Self: Sized,
    {
        writer.write_u8(if self { 1 } else { 0 })?;
        Ok(1)
    }
}

impl PacketContent for String {
    fn read<Reader: BufRead>(reader: &mut Reader) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        let len = VarInt::read(reader)?;
        let mut buf = Vec::with_capacity(len.0 as usize);
        reader.take(len.0 as u64).read_to_end(&mut buf)?;
        Ok(String::from_utf8_lossy(buf.as_ref()).into_owned())
    }

    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<usize>
    where
        Self: Sized,
    {
        let length = self.len();
        let len = VarInt(self.len() as i32);
        let var_int_size = VarInt::write(len, writer)?;
        writer.write_all(self.as_bytes())?;
        Ok(length + var_int_size)
    }
}

impl<Typ> PacketContent for Vec<Typ>
where
    Typ: PacketContent,
{
    fn read<Reader: BufRead>(reader: &mut Reader) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        let len = VarInt::read(reader)?;
        let mut vec = Vec::with_capacity(len.0 as usize);
        for _ in 0..len.0 {
            vec.push(Typ::read(reader)?);
        }
        Ok(vec)
    }
    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<usize>
    where
        Self: Sized,
    {
        let len = VarInt(self.len() as i32);
        let mut size = VarInt::write(len, writer)?;
        for item in self {
            size += item.write(writer)?;
        }
        Ok(size)
    }
}

impl<Typ> PacketContent for Option<Typ>
where
    Typ: PacketContent,
{
    fn read<Reader: BufRead>(reader: &mut Reader) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        let i = VarInt::read(reader)?;
        if i.0 == 0 {
            Ok(None)
        } else {
            Ok(Some(Typ::read(reader)?))
        }
    }
    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<usize>
    where
        Self: Sized,
    {
        let mut length = 0;
        if let Some(item) = self {
            let mut content = Vec::with_capacity(1);
            item.write(&mut content)?;
            length = VarInt(content.len() as i32).write(writer)?;
            length += content.len();
            writer.write_all(&mut content)?;
        } else {
            writer.write_u8(0)?;
            length = 1;
        }
        Ok(length)
    }
}

impl PacketContent for Uuid {
    fn read<Reader: BufRead>(reader: &mut Reader) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        let value = reader.read_u128::<BigEndian>()?;
        Ok(Uuid::from_u128(value))
    }

    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<usize>
    where
        Self: Sized,
    {
        writer.write_u128::<BigEndian>(self.as_u128())?;
        Ok(16)
    }
}

impl PacketContent for bytes::Bytes {
    fn read<Reader: BufRead>(reader: &mut Reader) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        let len = VarInt::read(reader)?;

        let mut buffer = Vec::with_capacity(len.0 as usize);
        reader.take(len.0 as u64).read_to_end(&mut buffer)?;
        Ok(bytes::Bytes::from(buffer))
    }

    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<usize>
    where
        Self: Sized,
    {
        todo!()
    }
}
