mod cb_packet_server_info { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketServerInfo ; impl Packet for CbPacketServerInfo { type PacketIDType = i32 ; type PacketContent = PacketServerInfoContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 0 } } pub struct PacketServerInfoContent { response: String ,

 } impl PacketContent for PacketServerInfoContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.response.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let response : String = PacketContent :: read ( reader ) ?;;

 Ok ( Self { response } ) } }

 }

 pub use cb_packet_server_info ::*;
