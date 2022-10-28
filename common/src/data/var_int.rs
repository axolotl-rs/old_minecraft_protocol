use crate::protocol::PacketContent;

use std::fmt::{Debug, Display};
use std::io::{BufRead, Write};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VarInt(pub i32);

impl PacketContent for VarInt {
    fn read<R: BufRead>(buf: &mut R) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        inline::read(buf)
    }

    fn write<W: Write>(self, write: &mut W) -> std::io::Result<usize> {
        inline::write(self, write)
    }
}

impl FromStr for VarInt {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        i32::from_str(s).map(VarInt)
    }
}

impl From<i32> for VarInt {
    fn from(value: i32) -> Self {
        VarInt(value)
    }
}

impl Into<i32> for VarInt {
    fn into(self) -> i32 {
        self.0
    }
}

impl PartialEq<i32> for VarInt {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl Display for VarInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Allows you to inline the VarInt read and write into the packet
/// It is a very marginal performance difference but could be worth it
/// https://nnethercote.github.io/perf-book/inlining.html
pub mod inline {
    use crate::data::VarInt;
    use byteorder::ReadBytesExt;
    use std::io::{BufRead, Write};

    #[inline(always)]
    pub fn read<R: BufRead>(buf: &mut R) -> std::io::Result<VarInt> {
        let mut number_of_reads = 0;
        let mut result = 0;
        loop {
            let read = buf.read_u8()?;

            let value = i32::from(read & 0x7F);
            result |= value.overflowing_shl(7 * number_of_reads).0;

            number_of_reads += 1;
            if number_of_reads > 5 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "VarInt too long",
                ));
            }
            if read & 0x80 == 0 {
                break;
            }
        }
        Ok(VarInt(result))
    }

    #[inline(always)]
    pub fn write<W: Write, VI: Into<i32>>(var_int: VI, write: &mut W) -> std::io::Result<usize> {
        let mut x = var_int.into();
        let mut iterations = 0;
        loop {
            let mut temp = (x & 0x7F) as u8;
            x >>= 7;
            if x != 0 {
                temp |= 0x80;
            }

            write.write_all(&[temp])?;

            iterations += 1;
            if x == 0 {
                break;
            }
        }
        Ok(iterations)
    }
}
