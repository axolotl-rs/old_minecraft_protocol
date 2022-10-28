use minecraft_protocol::{
    ClientBoundPlay, PacketReadError, PacketReader, PacketWriteError, PacketWriter, ServerBoundPlay,
};
use std::io::{BufRead, Write};
use axolotl_nbt::value::Value;
use minecraft_protocol::data::nbt::Nbt;
use minecraft_protocol::data::VarInt;
use minecraft_protocol::packets::play;
use minecraft_protocol::protocol::{Packet, PacketContent};
use crate::protocol::play::cb_packet_login::{CbPacketLogin, PacketLoginContent, PacketLoginContentArray};

pub struct PlayPacket;

impl minecraft_protocol::PlayPacket for PlayPacket {
    type ServerBound = SBPlayPacket;
    type ClientBound = CBPlayPacket;
}

pub enum SBPlayPacket {}

impl PacketReader for SBPlayPacket {
    fn read<R: BufRead>(packet_id: i32, reader: &mut R) -> Result<Self, PacketReadError>
        where
            Self: Sized,
    {
        todo!()
    }
}


impl Into<play::ServerBoundPlay> for SBPlayPacket {
    fn into(self) -> play::ServerBoundPlay {
        todo!()
    }
}

impl ServerBoundPlay for SBPlayPacket {}

#[derive(Debug, Clone, PartialEq)]
pub enum CBPlayPacket {
    PlayPacket(PacketLoginContent)
}

impl PacketWriter for CBPlayPacket {
    fn write<Writer: Write>(self, writer: &mut Writer) -> Result<usize, PacketWriteError> {
        match self {
            CBPlayPacket::PlayPacket(packet) => {
                let mut total_bytes = VarInt::from(CbPacketLogin::packet_id()).write(writer)?;
                total_bytes += packet.write(writer)?;
                Ok(total_bytes)
            }
        }
    }
}


impl From<play::ClientBoundPlay> for CBPlayPacket {
    fn from(v: play::ClientBoundPlay) -> Self {
        match v {
            play::ClientBoundPlay::PlayPacket {
                entity_id, is_hardcore, game_mode, previous_game_mode, world_names, registry_codec, dimension_type, world_name, hashed_seed, max_players, view_distance, simulation_distance, reduced_debug_info, enable_respawn_screen, is_debug, is_flat
            } => {
                CBPlayPacket::PlayPacket(PacketLoginContent {
                    entity_id,
                    is_hardcore,
                    game_mode,
                    previous_game_mode,
                    world_names,
                    registry_codec,
                    dimension_type,
                    world_name,
                    hashed_seed,
                    max_players,
                    view_distance,
                    simulation_distance,
                    reduced_debug_info,
                    enable_respawn_screen,
                    is_debug,
                    is_flat,
                })
            }
            _ => todo!()
        }
    }
}

impl ClientBoundPlay for CBPlayPacket {}
