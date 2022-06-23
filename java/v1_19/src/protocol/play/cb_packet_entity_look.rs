mod cb_packet_entity_look { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketEntityLook ; impl Packet for CbPacketEntityLook { type PacketIDType = i32 ; type PacketContent = PacketEntityLookContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 43 } } pub struct PacketEntityLookContent { entity_id: minecraft_data::common::data::VarInt ,

yaw: i8 ,

pitch: i8 ,

on_ground: bool ,

 } impl PacketContent for PacketEntityLookContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.entity_id.write(writer)?;;

total_bytes += self.yaw.write(writer)?;;

total_bytes += self.pitch.write(writer)?;;

total_bytes += self.on_ground.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let entity_id : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let yaw : i8 = PacketContent :: read ( reader ) ?;;

let pitch : i8 = PacketContent :: read ( reader ) ?;;

let on_ground : bool = PacketContent :: read ( reader ) ?;;

 Ok ( Self { entity_id, yaw, pitch, on_ground } ) } }

 }

 pub use cb_packet_entity_look ::*;
