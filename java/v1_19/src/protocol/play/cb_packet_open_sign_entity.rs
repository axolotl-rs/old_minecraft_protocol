mod cb_packet_open_sign_entity { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketOpenSignEntity ; impl Packet for CbPacketOpenSignEntity { type PacketIDType = i32 ; type PacketContent = PacketOpenSignEntityContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 47 } } pub struct PacketOpenSignEntityContent { location: minecraft_data::common::data::position::Position ,

 } impl PacketContent for PacketOpenSignEntityContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.location.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let location : minecraft_data::common::data::position::Position = PacketContent :: read ( reader ) ?;;

 Ok ( Self { location } ) } }

 }

 pub use cb_packet_open_sign_entity ::*;
