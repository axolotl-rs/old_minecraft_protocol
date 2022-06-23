mod cb_packet_end_combat_event { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketEndCombatEvent ; impl Packet for CbPacketEndCombatEvent { type PacketIDType = i32 ; type PacketContent = PacketEndCombatEventContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 51 } } pub struct PacketEndCombatEventContent { duration: minecraft_data::common::data::VarInt ,

entity_id: i32 ,

 } impl PacketContent for PacketEndCombatEventContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.duration.write(writer)?;;

total_bytes += self.entity_id.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let duration : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let entity_id : i32 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { duration, entity_id } ) } }

 }

 pub use cb_packet_end_combat_event ::*;
