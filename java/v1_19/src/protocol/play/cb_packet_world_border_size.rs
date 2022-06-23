mod cb_packet_world_border_size { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketWorldBorderSize ; impl Packet for CbPacketWorldBorderSize { type PacketIDType = i32 ; type PacketContent = PacketWorldBorderSizeContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 68 } } pub struct PacketWorldBorderSizeContent { diameter: f64 ,

 } impl PacketContent for PacketWorldBorderSizeContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.diameter.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let diameter : f64 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { diameter } ) } }

 }

 pub use cb_packet_world_border_size ::*;
