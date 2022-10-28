use crate::protocol::PacketContent;
use std::io::{BufRead, Write};
use axolotl_nbt::NBTDataType;
use axolotl_nbt::value::Value;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct OptionalNbt(pub Option<Value>);

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

#[derive(PartialEq, Clone, Debug)]
pub struct Nbt(pub Value);

impl PacketContent for Nbt {
    fn read<Reader: BufRead>(reader: &mut Reader) -> std::io::Result<Self>
        where
            Self: Sized,
    {
        let nbt = Value::read(reader).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        Ok(Self(nbt))
    }

    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<usize>
        where
            Self: Sized,
    {
        let mut vec = Vec::new();
        self.0.write_alone(&mut vec).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        let bytes_written = vec.len();
        writer.write_all(&vec)?;
        Ok(bytes_written)
    }
}
