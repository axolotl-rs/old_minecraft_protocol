use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketRecipeBook ; impl Packet for SbPacketRecipeBook { type PacketIDType = i32 ; type PacketContent = PacketRecipeBookContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 30 } } pub struct PacketRecipeBookContent { pub book_id: minecraft_data::data::VarInt ,

pub book_open: bool ,

pub filter_active: bool ,

 } impl PacketContent for PacketRecipeBookContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.book_id.write(writer)?;;

total_bytes += self.book_open.write(writer)?;;

total_bytes += self.filter_active.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let book_id : minecraft_data::data::VarInt = PacketContent :: read ( reader ) ?;;

let book_open : bool = PacketContent :: read ( reader ) ?;;

let filter_active : bool = PacketContent :: read ( reader ) ?;;

 Ok ( Self { book_id, book_open, filter_active } ) } }
