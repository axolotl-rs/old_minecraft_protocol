mod sb_packet_edit_book { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketEditBook ; impl Packet for SbPacketEditBook { type PacketIDType = i32 ; type PacketContent = PacketEditBookContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 11 } } pub struct PacketEditBookContent { hand: minecraft_data::common::data::VarInt ,

pages: PacketEditBookContentArray ,

title: void ,

 } impl PacketContent for PacketEditBookContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.hand.write(writer)?;;

total_bytes += self.pages.write(writer)?;;

total_bytes += self.title.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let hand : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let pages : PacketEditBookContentArray = PacketContent :: read ( reader ) ?;;

let title : void = PacketContent :: read ( reader ) ?;;

 Ok ( Self { hand, pages, title } ) } } pub type PacketEditBookContentArray = Vec <String >;

 }

 pub use sb_packet_edit_book ::*;
