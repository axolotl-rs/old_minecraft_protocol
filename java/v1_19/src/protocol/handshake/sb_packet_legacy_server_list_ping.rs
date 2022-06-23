mod sb_packet_legacy_server_list_ping { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketLegacyServerListPing ; impl Packet for SbPacketLegacyServerListPing { type PacketIDType = i32 ; type PacketContent = PacketLegacyServerListPingContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 254 } } pub struct PacketLegacyServerListPingContent { payload: u8 ,

 } impl PacketContent for PacketLegacyServerListPingContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.payload.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let payload : u8 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { payload } ) } }

 }

 pub use sb_packet_legacy_server_list_ping ::*;
