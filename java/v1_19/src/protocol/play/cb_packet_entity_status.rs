mod cb_packet_entity_status { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketEntityStatus ; impl Packet for CbPacketEntityStatus { type PacketIDType = i32 ; type PacketContent = PacketEntityStatusContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 27 } } pub struct PacketEntityStatusContent { entity_id: i32 ,

entity_status: i8 ,

 } impl PacketContent for PacketEntityStatusContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.entity_id.write(writer)?;;

total_bytes += self.entity_status.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let entity_id : i32 = PacketContent :: read ( reader ) ?;;

let entity_status : i8 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { entity_id, entity_status } ) } }

 }

 pub use cb_packet_entity_status ::*;
