use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketLogin ; impl Packet for CbPacketLogin { type PacketIDType = i32 ; type PacketContent = PacketLoginContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 38 } } pub struct PacketLoginContent { entity_id: i32 ,

is_hardcore: bool ,

game_mode: u8 ,

previous_game_mode: i8 ,

world_names: PacketLoginContentArray ,

dimension_codec: minecraft_data::data::nbt::Nbt ,

dimension: minecraft_data::data::nbt::Nbt ,

world_name: String ,

hashed_seed: i64 ,

max_players: minecraft_data::data::VarInt ,

view_distance: minecraft_data::data::VarInt ,

simulation_distance: minecraft_data::data::VarInt ,

reduced_debug_info: bool ,

enable_respawn_screen: bool ,

is_debug: bool ,

is_flat: bool ,

 } impl PacketContent for PacketLoginContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.entity_id.write(writer)?;;

total_bytes += self.is_hardcore.write(writer)?;;

total_bytes += self.game_mode.write(writer)?;;

total_bytes += self.previous_game_mode.write(writer)?;;

total_bytes += self.world_names.write(writer)?;;

total_bytes += self.dimension_codec.write(writer)?;;

total_bytes += self.dimension.write(writer)?;;

total_bytes += self.world_name.write(writer)?;;

total_bytes += self.hashed_seed.write(writer)?;;

total_bytes += self.max_players.write(writer)?;;

total_bytes += self.view_distance.write(writer)?;;

total_bytes += self.simulation_distance.write(writer)?;;

total_bytes += self.reduced_debug_info.write(writer)?;;

total_bytes += self.enable_respawn_screen.write(writer)?;;

total_bytes += self.is_debug.write(writer)?;;

total_bytes += self.is_flat.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let entity_id : i32 = PacketContent :: read ( reader ) ?;;

let is_hardcore : bool = PacketContent :: read ( reader ) ?;;

let game_mode : u8 = PacketContent :: read ( reader ) ?;;

let previous_game_mode : i8 = PacketContent :: read ( reader ) ?;;

let world_names : PacketLoginContentArray = PacketContent :: read ( reader ) ?;;

let dimension_codec : minecraft_data::data::nbt::Nbt = PacketContent :: read ( reader ) ?;;

let dimension : minecraft_data::data::nbt::Nbt = PacketContent :: read ( reader ) ?;;

let world_name : String = PacketContent :: read ( reader ) ?;;

let hashed_seed : i64 = PacketContent :: read ( reader ) ?;;

let max_players : minecraft_data::data::VarInt = PacketContent :: read ( reader ) ?;;

let view_distance : minecraft_data::data::VarInt = PacketContent :: read ( reader ) ?;;

let simulation_distance : minecraft_data::data::VarInt = PacketContent :: read ( reader ) ?;;

let reduced_debug_info : bool = PacketContent :: read ( reader ) ?;;

let enable_respawn_screen : bool = PacketContent :: read ( reader ) ?;;

let is_debug : bool = PacketContent :: read ( reader ) ?;;

let is_flat : bool = PacketContent :: read ( reader ) ?;;

 Ok ( Self { entity_id, is_hardcore, game_mode, previous_game_mode, world_names, dimension_codec, dimension, world_name, hashed_seed, max_players, view_distance, simulation_distance, reduced_debug_info, enable_respawn_screen, is_debug, is_flat } ) } } pub type PacketLoginContentArray = Vec <String >;
