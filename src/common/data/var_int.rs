use std::io::{BufRead, Write};
use crate::common::protocol::PacketContent;
use byteorder::ReadBytesExt;
pub struct VarInt(pub i32);
impl PacketContent for VarInt {
    fn read<R: BufRead>(buf: &mut R) -> std::io::Result<Self>
        where
            Self: Sized,
    {
        let mut number_of_reads = 0;
        let mut result = 0;
        loop {
            let read = buf.read_u8()?;

            let value = i32::from(read & 0x7F);
            result |= value.overflowing_shl(7 * number_of_reads).0;

            number_of_reads += 1;
            if number_of_reads > 5 {
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "VarInt too long"));
            }
            if read & 0x80 == 0 {
                break;
            }
        }
        Ok(VarInt(result))
    }

    fn write<W: Write>(
        self,
        write: &mut W) ->std::io::Result<usize>{
        let mut x = self.0 as u32;
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