use bytes::Bytes;
use minecraft_protocol::data::VarInt;
use minecraft_protocol::protocol::PacketContent;
use std::io::{BufRead, Read, Write};

pub mod cb_packet_compress;
pub mod cb_packet_disconnect;
pub mod cb_packet_encryption_begin;
//pub mod cb_packet_login_plugin_request;
pub mod cb_packet_success;
pub mod sb_packet_encryption_begin;
//pub mod sb_packet_login_plugin_response;
pub mod sb_packet_login_start;

pub struct SigData {
    pub pub_key_length: VarInt,
    pub pub_key: Bytes,
    pub signature_length: VarInt,
    pub signature: Bytes,
}

impl PacketContent for SigData {
    fn read<Reader: BufRead>(reader: &mut Reader) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        let pub_key_len = VarInt::read(reader)?;
        let mut pub_key = Vec::with_capacity(pub_key_len.0 as usize);
        reader
            .take(pub_key_len.0 as u64)
            .read_to_end(&mut pub_key)?;

        let signature_length = VarInt::read(reader)?;
        let mut signature = Vec::with_capacity(signature_length.0 as usize);
        reader
            .take(signature_length.0 as u64)
            .read_to_end(&mut signature)?;

        Ok(Self {
            pub_key_length: pub_key_len,
            pub_key: Bytes::from(pub_key),
            signature_length,
            signature: Bytes::from(signature),
        })
    }

    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<usize>
    where
        Self: Sized,
    {
        let mut total_bytes = 0;
        total_bytes += self.pub_key_length.write(writer)?;
        total_bytes += writer.write(self.pub_key.as_ref())?;
        total_bytes += self.signature_length.write(writer)?;
        total_bytes += writer.write(self.signature.as_ref())?;
        Ok(total_bytes)
    }
}
