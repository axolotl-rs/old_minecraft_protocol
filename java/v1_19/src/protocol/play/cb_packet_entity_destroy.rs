mod cb_packet_entity_destroy { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketEntityDestroy ; impl Packet for CbPacketEntityDestroy { type PacketIDType = i32 ; type PacketContent = PacketEntityDestroyContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 58 } } pub struct PacketEntityDestroyContent { entity_ids: PacketEntityDestroyContentArray ,

 } impl PacketContent for PacketEntityDestroyContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.entity_ids.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let entity_ids : PacketEntityDestroyContentArray = PacketContent :: read ( reader ) ?;;

 Ok ( Self { entity_ids } ) } } pub type PacketEntityDestroyContentArray = Vec <minecraft_data::common::data::VarInt >;

 }

 pub use cb_packet_entity_destroy ::*;
