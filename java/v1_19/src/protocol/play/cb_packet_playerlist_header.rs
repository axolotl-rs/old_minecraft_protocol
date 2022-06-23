mod cb_packet_playerlist_header { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketPlayerlistHeader ; impl Packet for CbPacketPlayerlistHeader { type PacketIDType = i32 ; type PacketContent = PacketPlayerlistHeaderContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 95 } } pub struct PacketPlayerlistHeaderContent { header: String ,

footer: String ,

 } impl PacketContent for PacketPlayerlistHeaderContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.header.write(writer)?;;

total_bytes += self.footer.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let header : String = PacketContent :: read ( reader ) ?;;

let footer : String = PacketContent :: read ( reader ) ?;;

 Ok ( Self { header, footer } ) } }

 }

 pub use cb_packet_playerlist_header ::*;
