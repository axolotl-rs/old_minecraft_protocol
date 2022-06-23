mod cb_packet_remove_entity_effect { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketRemoveEntityEffect ; impl Packet for CbPacketRemoveEntityEffect { type PacketIDType = i32 ; type PacketContent = PacketRemoveEntityEffectContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 59 } } pub struct PacketRemoveEntityEffectContent { entity_id: minecraft_data::common::data::VarInt ,

effect_id: minecraft_data::common::data::VarInt ,

 } impl PacketContent for PacketRemoveEntityEffectContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.entity_id.write(writer)?;;

total_bytes += self.effect_id.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let entity_id : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let effect_id : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

 Ok ( Self { entity_id, effect_id } ) } }

 }

 pub use cb_packet_remove_entity_effect ::*;
