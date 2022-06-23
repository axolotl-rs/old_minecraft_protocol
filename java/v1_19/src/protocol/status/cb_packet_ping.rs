mod cb_packet_ping { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketPing ; impl Packet for CbPacketPing { type PacketIDType = i32 ; type PacketContent = PacketPingContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 1 } } pub struct PacketPingContent { time: i64 ,

 } impl PacketContent for PacketPingContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.time.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let time : i64 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { time } ) } }

 }

 pub use cb_packet_ping ::*;
