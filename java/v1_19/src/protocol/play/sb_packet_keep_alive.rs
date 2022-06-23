mod sb_packet_keep_alive { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketKeepAlive ; impl Packet for SbPacketKeepAlive { type PacketIDType = i32 ; type PacketContent = PacketKeepAliveContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 15 } } pub struct PacketKeepAliveContent { keep_alive_id: i64 ,

 } impl PacketContent for PacketKeepAliveContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.keep_alive_id.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let keep_alive_id : i64 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { keep_alive_id } ) } }

 }

 pub use sb_packet_keep_alive ::*;
