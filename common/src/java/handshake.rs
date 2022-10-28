use std::io::{BufRead, Write};

use crate::packets::handshake::{SbPacketLegacyServerListPing, SetProtocol};
use crate::protocol::{Packet, PacketContent};
use crate::{HandshakePacket, PacketReadError, PacketReader, ServerBoundHandShake};

pub struct HandshakePackets;

impl HandshakePacket for HandshakePackets {
    type ServerBound = SBHandshakePackets;
    type ClientBound = ();
}

pub enum SBHandshakePackets {
    SetProtocol(<SetProtocol as Packet>::PacketContent),
    LegacyServerListPing(<SbPacketLegacyServerListPing as Packet>::PacketContent),
}

impl PacketReader for SBHandshakePackets {
    fn read<R: BufRead>(packet_id: i32, reader: &mut R) -> Result<Self, PacketReadError>
    where
        Self: Sized,
    {
        if packet_id == SetProtocol::packet_id() {
            let packet = <SetProtocol as Packet>::PacketContent::read(reader)?;
            Ok(Self::SetProtocol(packet))
        } else if packet_id == SbPacketLegacyServerListPing::packet_id() {
            let packet = <SbPacketLegacyServerListPing as Packet>::PacketContent::read(reader)?;
            Ok(Self::LegacyServerListPing(packet))
        } else {
            Err(PacketReadError::MalformedPacket("Unknown packet id"))
        }
    }
}

impl ServerBoundHandShake for SBHandshakePackets {}
