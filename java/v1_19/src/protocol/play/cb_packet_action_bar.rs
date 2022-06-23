mod cb_packet_action_bar { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketActionBar ; impl Packet for CbPacketActionBar { type PacketIDType = i32 ; type PacketContent = PacketActionBarContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 65 } } pub struct PacketActionBarContent { text: String ,

 } impl PacketContent for PacketActionBarContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.text.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let text : String = PacketContent :: read ( reader ) ?;;

 Ok ( Self { text } ) } }

 }

 pub use cb_packet_action_bar ::*;
