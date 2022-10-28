use crate::protocol::login::cb_packet_compress::PacketCompressContent;
use crate::protocol::login::cb_packet_disconnect::PacketDisconnectContent;
use crate::protocol::login::cb_packet_encryption_begin::{
    CbPacketEncryptionBegin, PacketEncryptionBeginContent,
};
use crate::protocol::login::cb_packet_success::LoginSuccess;
use crate::protocol::login::sb_packet_encryption_begin;
use crate::protocol::login::sb_packet_encryption_begin::SbPacketEncryptionBegin;
use crate::protocol::login::sb_packet_login_start::{PacketLoginStartContent, SbPacketLoginStart};
use minecraft_protocol::data::VarInt;
use minecraft_protocol::protocol::{Packet, PacketContent};
use minecraft_protocol::{
    ClientBoundLogin, LoginPacket, PacketReadError, PacketReader, PacketWriteError, PacketWriter,
    ServerBoundLogin,
};
use std::io::{BufRead, Write};

pub struct LoginPackets;

impl LoginPacket for LoginPackets {
    type ServerBound = SBLogin;
    type ClientBound = CBLogin;
}

pub enum SBLogin {
    PacketEncryptionBegin(PacketEncryptionBeginContent),
    //TODO Support Plugin Channels
    PacketLoginPluginResponse,
    PacketLoginStart(PacketLoginStartContent),
}

impl PacketReader for SBLogin {
    fn read<R: BufRead>(packet_id: i32, reader: &mut R) -> Result<Self, PacketReadError>
    where
        Self: Sized,
    {
        if packet_id == sb_packet_encryption_begin::SbPacketEncryptionBegin::packet_id() {
            let packet = PacketEncryptionBeginContent::read(reader)?;
            Ok(SBLogin::PacketEncryptionBegin(packet))
        } else if packet_id == SbPacketLoginStart::packet_id() {
            let packet = PacketLoginStartContent::read(reader)?;
            Ok(SBLogin::PacketLoginStart(packet))
        } else {
            println!("Unknown packet id: {}", packet_id);
            Err(PacketReadError::MalformedPacket("Unknown packet id"))
        }
    }
}

impl ServerBoundLogin for SBLogin {}

pub enum CBLogin {
    PacketDisconnect(PacketDisconnectContent),
    PacketSuccess(LoginSuccess),
    PacketCompress(PacketCompressContent),
    PacketEncryptionBegin(CbPacketEncryptionBegin),
    PacketPluginRequest,
}

impl PacketWriter for CBLogin {
    fn write<Writer: Write>(self, writer: &mut Writer) -> Result<usize, PacketWriteError> {
        todo!()
    }
}

impl ClientBoundLogin for CBLogin {}
