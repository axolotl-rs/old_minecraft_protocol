mod sb_packet_chat { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketChat ; impl Packet for SbPacketChat { type PacketIDType = i32 ; type PacketContent = PacketChatContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 3 } } pub struct PacketChatContent { message: String ,

 } impl PacketContent for PacketChatContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.message.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let message : String = PacketContent :: read ( reader ) ?;;

 Ok ( Self { message } ) } }

 }

 pub use sb_packet_chat ::*;
