use axolotl_nbt::value::Value;
use crate::data::nbt::Nbt;
use crate::data::VarInt;

/// The Non Version Specific Packet Input
#[derive(Debug, Clone, PartialEq)]
pub enum ClientBoundPlay {
    PlayerPosition {
        x: f64,
        y: f64,
        z: f64,
        on_ground: bool,
    },
    PlayPacket {
        entity_id: i32,
        is_hardcore: bool,
        game_mode: u8,
        previous_game_mode: i8,
        world_names: Vec<String>,
        registry_codec: Nbt<Value>,
        dimension_type: String,
        world_name: String,
        hashed_seed: i64,
        max_players: VarInt,
        view_distance: VarInt,
        simulation_distance: VarInt,
        reduced_debug_info: bool,
        enable_respawn_screen: bool,
        is_debug: bool,
        is_flat: bool,
    },
}