mod sb_packet_pong { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketPong ; impl Packet for SbPacketPong { type PacketIDType = i32 ; type PacketContent = PacketPongContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 29 } } pub struct PacketPongContent { id: i32 ,

 } impl PacketContent for PacketPongContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.id.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let id : i32 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { id } ) } }

 }

 pub use sb_packet_pong ::*;
