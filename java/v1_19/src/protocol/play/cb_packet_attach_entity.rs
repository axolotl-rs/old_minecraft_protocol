mod cb_packet_attach_entity { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketAttachEntity ; impl Packet for CbPacketAttachEntity { type PacketIDType = i32 ; type PacketContent = PacketAttachEntityContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 78 } } pub struct PacketAttachEntityContent { entity_id: i32 ,

vehicle_id: i32 ,

 } impl PacketContent for PacketAttachEntityContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.entity_id.write(writer)?;;

total_bytes += self.vehicle_id.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let entity_id : i32 = PacketContent :: read ( reader ) ?;;

let vehicle_id : i32 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { entity_id, vehicle_id } ) } }

 }

 pub use cb_packet_attach_entity ::*;
