use std::fmt::Debug;
use crate::protocol::PacketContent;
use axolotl_nbt::value::Value;
use axolotl_nbt::{NBTDataType, serde_impl};
use std::io::{BufRead, Write};
use serde::de::DeserializeOwned;
use serde::Serialize;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct OptionalNbt<V: DeserializeOwned + Serialize>(pub Option<V>);

impl<V: DeserializeOwned + Serialize> PacketContent for OptionalNbt<V> {
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
pub enum Nbt<V> {
    AsNBT(V),
    AsBuffer(Vec<u8>),
}

impl<V: DeserializeOwned + Serialize> PacketContent for Nbt<V> {
    fn read<Reader: BufRead>(reader: &mut Reader) -> std::io::Result<Self>
        where
            Self: Sized,
    {
        let nbt =
            serde_impl::from_buf_reader_binary(reader).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        Ok(Self::AsBuffer(nbt))
    }

    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<usize>
        where
            Self: Sized,
    {
        let vec = match self {
            Nbt::AsNBT(bytes) => {
                let mut vec = Vec::new();
                serde_impl::to_writer(&mut vec, &bytes).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
                vec
            }
            Nbt::AsBuffer(v) => {
                v
            }
        };
        let bytes_written = vec.len();
        writer.write_all(&vec)?;
        Ok(bytes_written)
    }
}
