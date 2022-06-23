mod sb_packet_look { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketLook ; impl Packet for SbPacketLook { type PacketIDType = i32 ; type PacketContent = PacketLookContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 19 } } pub struct PacketLookContent { yaw: f32 ,

pitch: f32 ,

on_ground: bool ,

 } impl PacketContent for PacketLookContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.yaw.write(writer)?;;

total_bytes += self.pitch.write(writer)?;;

total_bytes += self.on_ground.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let yaw : f32 = PacketContent :: read ( reader ) ?;;

let pitch : f32 = PacketContent :: read ( reader ) ?;;

let on_ground : bool = PacketContent :: read ( reader ) ?;;

 Ok ( Self { yaw, pitch, on_ground } ) } }

 }

 pub use sb_packet_look ::*;
