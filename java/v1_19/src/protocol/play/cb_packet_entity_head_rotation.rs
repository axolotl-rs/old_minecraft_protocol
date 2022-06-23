mod cb_packet_entity_head_rotation { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketEntityHeadRotation ; impl Packet for CbPacketEntityHeadRotation { type PacketIDType = i32 ; type PacketContent = PacketEntityHeadRotationContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 62 } } pub struct PacketEntityHeadRotationContent { entity_id: minecraft_data::common::data::VarInt ,

head_yaw: i8 ,

 } impl PacketContent for PacketEntityHeadRotationContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.entity_id.write(writer)?;;

total_bytes += self.head_yaw.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let entity_id : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let head_yaw : i8 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { entity_id, head_yaw } ) } }

 }

 pub use cb_packet_entity_head_rotation ::*;
