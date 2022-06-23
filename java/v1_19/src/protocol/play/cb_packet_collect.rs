mod cb_packet_collect { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketCollect ; impl Packet for CbPacketCollect { type PacketIDType = i32 ; type PacketContent = PacketCollectContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 97 } } pub struct PacketCollectContent { collected_entity_id: minecraft_data::common::data::VarInt ,

collector_entity_id: minecraft_data::common::data::VarInt ,

pickup_item_count: minecraft_data::common::data::VarInt ,

 } impl PacketContent for PacketCollectContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.collected_entity_id.write(writer)?;;

total_bytes += self.collector_entity_id.write(writer)?;;

total_bytes += self.pickup_item_count.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let collected_entity_id : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let collector_entity_id : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let pickup_item_count : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

 Ok ( Self { collected_entity_id, collector_entity_id, pickup_item_count } ) } }

 }

 pub use cb_packet_collect ::*;
