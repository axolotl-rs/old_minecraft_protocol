use crate::protocol::status::sb_packet_ping::{PacketPingContent, SbPacketPing};
use crate::protocol::status::sb_packet_ping_start::{PacketPingStartContent, SbPacketPingStart};
use crate::protocol::status::{cb_packet_ping, cb_packet_server_info};
use minecraft_protocol::protocol::{Packet, PacketContent};
use minecraft_protocol::{
    ClientBoundStatus, PacketReadError, PacketReader, PacketWriteError, PacketWriter,
    ServerBoundStatus,
};
use std::io::{BufRead, Write};
use minecraft_protocol::packets::status::client_bound::ClientBoundStatus as GenericClientBoundStatus;

pub struct StatusPacket;

impl minecraft_protocol::StatusPacket for StatusPacket {
    type ServerBound = SBStatusPackets;
    type ClientBound = CBStatusPackets;
}

pub enum SBStatusPackets {
    PacketPing(PacketPingContent),
    PacketPingStart(PacketPingStartContent),
}

impl PacketReader for SBStatusPackets {
    fn read<R: BufRead>(packet_id: i32, reader: &mut R) -> Result<Self, PacketReadError>
        where
            Self: Sized,
    {
        if packet_id == SbPacketPing::packet_id() {
            let packet = PacketPingContent::read(reader)?;
            Ok(Self::PacketPing(packet))
        } else if packet_id == SbPacketPingStart::packet_id() {
            let packet = PacketPingStartContent::read(reader)?;
            Ok(Self::PacketPingStart(packet))
        } else {
            println!("Unknown packet id: {}", packet_id);
            Err(PacketReadError::MalformedPacket("Unknown packet id"))
        }
    }
}

impl Into<minecraft_protocol::packets::status::server_bound::ServerBoundStatus> for SBStatusPackets {
    fn into(self) -> minecraft_protocol::packets::status::server_bound::ServerBoundStatus {
        match self {
            SBStatusPackets::PacketPing(packet) => {
                minecraft_protocol::packets::status::server_bound::ServerBoundStatus::Ping {
                    payload: packet.time,
                }
            }
            SBStatusPackets::PacketPingStart(_) => {
                minecraft_protocol::packets::status::server_bound::ServerBoundStatus::Request
            }
        }
    }
}

impl ServerBoundStatus for SBStatusPackets {}

pub enum CBStatusPackets {
    PacketPing(cb_packet_ping::PacketPingContent),
    PacketServerInfo(cb_packet_server_info::PacketServerInfoContent),
}

impl PacketWriter for CBStatusPackets {
    fn write<Writer: Write>(self, writer: &mut Writer) -> Result<usize, PacketWriteError> {
        match self {
            CBStatusPackets::PacketPing(packet) => packet.write(writer).map_err(PacketWriteError::IOError),
            CBStatusPackets::PacketServerInfo(packet) => packet.write(writer).map_err(PacketWriteError::IOError),
        }
    }
}

impl From<GenericClientBoundStatus> for CBStatusPackets {
    fn from(v: GenericClientBoundStatus) -> Self {
        match v {
            GenericClientBoundStatus::Response { response } => {
                CBStatusPackets::PacketServerInfo(cb_packet_server_info::PacketServerInfoContent {
                    response,
                })
            }
            GenericClientBoundStatus::Pong { payload } => {
                CBStatusPackets::PacketPing(cb_packet_ping::PacketPingContent { time: payload })
            }
        }
    }
}

impl ClientBoundStatus for CBStatusPackets {}
