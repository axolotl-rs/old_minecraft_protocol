use std::io;
use std::io::{BufRead, Write};
use std::str::FromStr;

pub trait Packet {
    type PacketIDType;
    type PacketContent: PacketContent;
    fn packet_id() -> Self::PacketIDType
        where Self: Sized;

    fn packet_content() -> Self::PacketContent
        where Self: Sized;
}

pub trait PacketContent {
    fn read<Reader: BufRead>(
        reader: &mut Reader,
    ) -> io::Result<Self>
        where
            Self: Sized;
    fn write<Writer: Write>(
        self,
        writer: &mut Writer,
    ) -> io::Result<usize>
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

#[cfg(test)]
pub mod tests {
    use std::io::{BufRead, Write};
    use std::str::FromStr;
    use crate::common::data::VarInt;
    use crate::common::protocol::{PacketContent, PacketSwitch};

    pub struct MyPacketContent {
        pub compare_to: VarInt,
        pub switch: MyGenericSwitch,
    }

    impl PacketContent for MyPacketContent {
        fn read<Reader: BufRead>(reader: &mut Reader) -> std::io::Result<Self> where Self: Sized {
            let compare_to = VarInt::read(reader)?;
            let switch = MyGenericSwitch::switch_read(&compare_to, reader)?;
            Ok(MyPacketContent {
                compare_to,
                switch,
            })
        }

        fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<usize> where Self: Sized {
            todo!()
        }
    }

    pub enum MyGenericSwitch {
        A,
        B,
    }
        fn switch_read<Reader: BufRead>(key: &Self::CompareType, reader: &mut Reader) -> std::io::Result<Self> where Self: Sized {
            if key.eq(&0) {
                Ok(MyGenericSwitch::A)
            } else {

                impl PacketSwitch for MyGenericSwitch {
                    type CompareType = VarInt;

                    Ok(MyGenericSwitch::B)
            }
        }

        fn switch_write<Writer: Write>(self, write_compare: bool, writer: &mut Writer) -> std::io::Result<usize> where Self: Sized {
            todo!()
        }
    }

    #[test]
    pub fn test() {}
}