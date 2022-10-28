use minecraft_protocol::data::nbt::Nbt;
use minecraft_protocol::protocol::Packet;
use minecraft_protocol::protocol::PacketContent;

use minecraft_protocol::data::VarInt;
use std::io::{BufRead, Write};
use axolotl_nbt::value::Value;

pub struct CbPacketLogin;

impl Packet for CbPacketLogin {
    type PacketIDType = i32;
    type PacketContent = PacketLoginContent;
    fn packet_id() -> Self::PacketIDType
    where
        Self: Sized,
    {
        38
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PacketLoginContent {
    pub entity_id: i32,
    pub is_hardcore: bool,
    pub game_mode: u8,
    pub previous_game_mode: i8,
    pub world_names: PacketLoginContentArray,
    pub registry_codec: Nbt<Value>,
    pub dimension_type: String,
    pub world_name: String,
    pub hashed_seed: i64,
    pub max_players: VarInt,
    pub view_distance: VarInt,
    pub simulation_distance: VarInt,
    pub reduced_debug_info: bool,
    pub enable_respawn_screen: bool,
    pub is_debug: bool,
    pub is_flat: bool,
}

impl PacketContent for PacketLoginContent {
    fn read<Reader: BufRead>(reader: &mut Reader) -> std::io::Result<Self> {
        let entity_id: i32 = PacketContent::read(reader)?;

        let is_hardcore: bool = PacketContent::read(reader)?;

        let game_mode: u8 = PacketContent::read(reader)?;

        let previous_game_mode: i8 = PacketContent::read(reader)?;

        let world_names: PacketLoginContentArray = PacketContent::read(reader)?;

        let registry_codec: Nbt<Value> = PacketContent::read(reader)?;
        let dimension_type: String = PacketContent::read(reader)?;
        let world_name: String = PacketContent::read(reader)?;

        let hashed_seed: i64 = PacketContent::read(reader)?;

        let max_players: VarInt = PacketContent::read(reader)?;

        let view_distance: VarInt = PacketContent::read(reader)?;

        let simulation_distance: VarInt = PacketContent::read(reader)?;

        let reduced_debug_info: bool = PacketContent::read(reader)?;

        let enable_respawn_screen: bool = PacketContent::read(reader)?;

        let is_debug: bool = PacketContent::read(reader)?;

        let is_flat: bool = PacketContent::read(reader)?;

        Ok(Self {
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
    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<usize> {
        let mut total_bytes = 0;
        total_bytes += self.entity_id.write(writer)?;

        total_bytes += self.is_hardcore.write(writer)?;

        total_bytes += self.game_mode.write(writer)?;

        total_bytes += self.previous_game_mode.write(writer)?;

        total_bytes += self.world_names.write(writer)?;

        total_bytes += self.registry_codec.write(writer)?;

        total_bytes += self.dimension_type.write(writer)?;

        total_bytes += self.world_name.write(writer)?;

        total_bytes += self.hashed_seed.write(writer)?;

        total_bytes += self.max_players.write(writer)?;

        total_bytes += self.view_distance.write(writer)?;

        total_bytes += self.simulation_distance.write(writer)?;

        total_bytes += self.reduced_debug_info.write(writer)?;

        total_bytes += self.enable_respawn_screen.write(writer)?;

        total_bytes += self.is_debug.write(writer)?;

        total_bytes += self.is_flat.write(writer)?;

        Ok(total_bytes)
    }
}

pub type PacketLoginContentArray = Vec<String>;
