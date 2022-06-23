use std::io;
use std::io::{BufRead, Write};
use std::str::FromStr;

pub trait Packet {
    type PacketIDType;
    type PacketContent: PacketContent;
    fn packet_id() -> Self::PacketIDType
    where
        Self: Sized;

    fn packet_content() -> Self::PacketContent
    where
        Self: Sized;
}

pub trait PacketContent {
    fn read<Reader: BufRead>(reader: &mut Reader) -> io::Result<Self>
    where
        Self: Sized;
    fn write<Writer: Write>(self, writer: &mut Writer) -> io::Result<usize>
    where
        Self: Sized;
}

pub trait PacketSwitch {
    type CompareType: FromStr;
    fn switch_read<Reader: BufRead>(
        key: &Self::CompareType,
        reader: &mut Reader,
    ) -> io::Result<Self>
    where
        Self: Sized;
    fn switch_write<Writer: Write>(
        self,
        write_compare: bool,
        writer: &mut Writer,
    ) -> io::Result<usize>
    where
        Self: Sized;
}
