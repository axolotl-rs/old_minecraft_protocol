mod sb_packet_name_item { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketNameItem ; impl Packet for SbPacketNameItem { type PacketIDType = i32 ; type PacketContent = PacketNameItemContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 32 } } pub struct PacketNameItemContent { name: String ,

 } impl PacketContent for PacketNameItemContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.name.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let name : String = PacketContent :: read ( reader ) ?;;

 Ok ( Self { name } ) } }

 }

 pub use sb_packet_name_item ::*;
