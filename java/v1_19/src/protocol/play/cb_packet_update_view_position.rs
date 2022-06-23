mod cb_packet_update_view_position { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketUpdateViewPosition ; impl Packet for CbPacketUpdateViewPosition { type PacketIDType = i32 ; type PacketContent = PacketUpdateViewPositionContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 73 } } pub struct PacketUpdateViewPositionContent { chunk_x: minecraft_data::common::data::VarInt ,

chunk_z: minecraft_data::common::data::VarInt ,

 } impl PacketContent for PacketUpdateViewPositionContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.chunk_x.write(writer)?;;

total_bytes += self.chunk_z.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let chunk_x : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let chunk_z : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

 Ok ( Self { chunk_x, chunk_z } ) } }

 }

 pub use cb_packet_update_view_position ::*;
