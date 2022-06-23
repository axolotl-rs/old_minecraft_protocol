mod cb_packet_update_view_distance { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketUpdateViewDistance ; impl Packet for CbPacketUpdateViewDistance { type PacketIDType = i32 ; type PacketContent = PacketUpdateViewDistanceContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 74 } } pub struct PacketUpdateViewDistanceContent { view_distance: minecraft_data::common::data::VarInt ,

 } impl PacketContent for PacketUpdateViewDistanceContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.view_distance.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let view_distance : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

 Ok ( Self { view_distance } ) } }

 }

 pub use cb_packet_update_view_distance ::*;
