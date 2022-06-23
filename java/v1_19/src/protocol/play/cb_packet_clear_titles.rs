mod cb_packet_clear_titles { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketClearTitles ; impl Packet for CbPacketClearTitles { type PacketIDType = i32 ; type PacketContent = PacketClearTitlesContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 16 } } pub struct PacketClearTitlesContent { reset: bool ,

 } impl PacketContent for PacketClearTitlesContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.reset.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let reset : bool = PacketContent :: read ( reader ) ?;;

 Ok ( Self { reset } ) } }

 }

 pub use cb_packet_clear_titles ::*;
