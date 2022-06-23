mod cb_packet_world_border_center { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketWorldBorderCenter ; impl Packet for CbPacketWorldBorderCenter { type PacketIDType = i32 ; type PacketContent = PacketWorldBorderCenterContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 66 } } pub struct PacketWorldBorderCenterContent { x: f64 ,

z: f64 ,

 } impl PacketContent for PacketWorldBorderCenterContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.x.write(writer)?;;

total_bytes += self.z.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let x : f64 = PacketContent :: read ( reader ) ?;;

let z : f64 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { x, z } ) } }

 }

 pub use cb_packet_world_border_center ::*;
