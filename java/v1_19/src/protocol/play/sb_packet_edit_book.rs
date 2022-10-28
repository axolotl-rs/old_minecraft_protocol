use minecraft_protocol :: protocol :: PacketContent ; use minecraft_protocol :: protocol :: PacketSwitch ; use minecraft_protocol :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketEditBook ; impl Packet for SbPacketEditBook { type PacketIDType = i32 ; type PacketContent = PacketEditBookContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 11 } } pub struct PacketEditBookContent { pub hand: minecraft_protocol::data::VarInt ,

pub pages: PacketEditBookContentArray ,

pub title: void ,

 } impl PacketContent for PacketEditBookContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.hand.write(writer)?;;

total_bytes += self.pages.write(writer)?;;

total_bytes += self.title.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let hand : minecraft_protocol::data::VarInt = PacketContent :: read ( reader ) ?;;

let pages : PacketEditBookContentArray = PacketContent :: read ( reader ) ?;;

let title : void = PacketContent :: read ( reader ) ?;;

 Ok ( Self { hand, pages, title } ) } } pub type PacketEditBookContentArray = Vec <String >;
