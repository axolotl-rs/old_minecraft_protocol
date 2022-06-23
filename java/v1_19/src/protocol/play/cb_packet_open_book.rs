mod cb_packet_open_book { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketOpenBook ; impl Packet for CbPacketOpenBook { type PacketIDType = i32 ; type PacketContent = PacketOpenBookContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 45 } } pub struct PacketOpenBookContent { hand: minecraft_data::common::data::VarInt ,

 } impl PacketContent for PacketOpenBookContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.hand.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let hand : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

 Ok ( Self { hand } ) } }

 }

 pub use cb_packet_open_book ::*;
