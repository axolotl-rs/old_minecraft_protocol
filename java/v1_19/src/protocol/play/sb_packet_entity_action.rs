mod sb_packet_entity_action { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketEntityAction ; impl Packet for SbPacketEntityAction { type PacketIDType = i32 ; type PacketContent = PacketEntityActionContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 27 } } pub struct PacketEntityActionContent { entity_id: minecraft_data::common::data::VarInt ,

action_id: minecraft_data::common::data::VarInt ,

jump_boost: minecraft_data::common::data::VarInt ,

 } impl PacketContent for PacketEntityActionContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.entity_id.write(writer)?;;

total_bytes += self.action_id.write(writer)?;;

total_bytes += self.jump_boost.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let entity_id : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let action_id : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let jump_boost : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

 Ok ( Self { entity_id, action_id, jump_boost } ) } }

 }

 pub use sb_packet_entity_action ::*;
