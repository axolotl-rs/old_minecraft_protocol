mod cb_packet_tile_entity_data { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketTileEntityData ; impl Packet for CbPacketTileEntityData { type PacketIDType = i32 ; type PacketContent = PacketTileEntityDataContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 10 } } pub struct PacketTileEntityDataContent { location: minecraft_data::common::data::position::Position ,

action: minecraft_data::common::data::VarInt ,

nbt_data: minecraft_data::common::data::nbt::OptionalNbt ,

 } impl PacketContent for PacketTileEntityDataContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.location.write(writer)?;;

total_bytes += self.action.write(writer)?;;

total_bytes += self.nbt_data.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let location : minecraft_data::common::data::position::Position = PacketContent :: read ( reader ) ?;;

let action : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let nbt_data : minecraft_data::common::data::nbt::OptionalNbt = PacketContent :: read ( reader ) ?;;

 Ok ( Self { location, action, nbt_data } ) } }

 }

 pub use cb_packet_tile_entity_data ::*;
