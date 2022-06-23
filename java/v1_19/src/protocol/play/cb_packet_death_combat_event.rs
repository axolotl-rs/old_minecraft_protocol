mod cb_packet_death_combat_event { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketDeathCombatEvent ; impl Packet for CbPacketDeathCombatEvent { type PacketIDType = i32 ; type PacketContent = PacketDeathCombatEventContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 53 } } pub struct PacketDeathCombatEventContent { player_id: minecraft_data::common::data::VarInt ,

entity_id: i32 ,

message: String ,

 } impl PacketContent for PacketDeathCombatEventContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.player_id.write(writer)?;;

total_bytes += self.entity_id.write(writer)?;;

total_bytes += self.message.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let player_id : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let entity_id : i32 = PacketContent :: read ( reader ) ?;;

let message : String = PacketContent :: read ( reader ) ?;;

 Ok ( Self { player_id, entity_id, message } ) } }

 }

 pub use cb_packet_death_combat_event ::*;
