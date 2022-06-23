mod sb_packet_login_start { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketLoginStart ; impl Packet for SbPacketLoginStart { type PacketIDType = i32 ; type PacketContent = PacketLoginStartContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 0 } } pub struct PacketLoginStartContent { username: String ,

 } impl PacketContent for PacketLoginStartContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.username.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let username : String = PacketContent :: read ( reader ) ?;;

 Ok ( Self { username } ) } }

 }

 pub use sb_packet_login_start ::*;
