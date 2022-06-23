mod cb_packet_respawn { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketRespawn ; impl Packet for CbPacketRespawn { type PacketIDType = i32 ; type PacketContent = PacketRespawnContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 61 } } pub struct PacketRespawnContent { dimension: minecraft_data::common::data::nbt::Nbt ,

world_name: String ,

hashed_seed: i64 ,

gamemode: u8 ,

previous_gamemode: u8 ,

is_debug: bool ,

is_flat: bool ,

copy_metadata: bool ,

 } impl PacketContent for PacketRespawnContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.dimension.write(writer)?;;

total_bytes += self.world_name.write(writer)?;;

total_bytes += self.hashed_seed.write(writer)?;;

total_bytes += self.gamemode.write(writer)?;;

total_bytes += self.previous_gamemode.write(writer)?;;

total_bytes += self.is_debug.write(writer)?;;

total_bytes += self.is_flat.write(writer)?;;

total_bytes += self.copy_metadata.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let dimension : minecraft_data::common::data::nbt::Nbt = PacketContent :: read ( reader ) ?;;

let world_name : String = PacketContent :: read ( reader ) ?;;

let hashed_seed : i64 = PacketContent :: read ( reader ) ?;;

let gamemode : u8 = PacketContent :: read ( reader ) ?;;

let previous_gamemode : u8 = PacketContent :: read ( reader ) ?;;

let is_debug : bool = PacketContent :: read ( reader ) ?;;

let is_flat : bool = PacketContent :: read ( reader ) ?;;

let copy_metadata : bool = PacketContent :: read ( reader ) ?;;

 Ok ( Self { dimension, world_name, hashed_seed, gamemode, previous_gamemode, is_debug, is_flat, copy_metadata } ) } }

 }

 pub use cb_packet_respawn ::*;
