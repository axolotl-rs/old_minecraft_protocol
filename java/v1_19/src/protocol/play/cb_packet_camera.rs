mod cb_packet_camera { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketCamera ; impl Packet for CbPacketCamera { type PacketIDType = i32 ; type PacketContent = PacketCameraContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 71 } } pub struct PacketCameraContent { camera_id: minecraft_data::common::data::VarInt ,

 } impl PacketContent for PacketCameraContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.camera_id.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let camera_id : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

 Ok ( Self { camera_id } ) } }

 }

 pub use cb_packet_camera ::*;
