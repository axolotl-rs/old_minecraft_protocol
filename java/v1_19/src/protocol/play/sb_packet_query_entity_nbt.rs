mod sb_packet_query_entity_nbt { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketQueryEntityNbt ; impl Packet for SbPacketQueryEntityNbt { type PacketIDType = i32 ; type PacketContent = PacketQueryEntityNbtContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 12 } } pub struct PacketQueryEntityNbtContent { transaction_id: minecraft_data::common::data::VarInt ,

entity_id: minecraft_data::common::data::VarInt ,

 } impl PacketContent for PacketQueryEntityNbtContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.transaction_id.write(writer)?;;

total_bytes += self.entity_id.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let transaction_id : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let entity_id : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

 Ok ( Self { transaction_id, entity_id } ) } }

 }

 pub use sb_packet_query_entity_nbt ::*;
