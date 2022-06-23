mod sb_packet_ping_start { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketPingStart ; impl Packet for SbPacketPingStart { type PacketIDType = i32 ; type PacketContent = PacketPingStartContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 0 } } pub struct PacketPingStartContent { } impl PacketContent for PacketPingStartContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { Ok ( Self { } ) } }

 }

 pub use sb_packet_ping_start ::*;
