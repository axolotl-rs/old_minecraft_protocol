use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketOpenBook ; impl Packet for CbPacketOpenBook { type PacketIDType = i32 ; type PacketContent = PacketOpenBookContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 45 } } pub struct PacketOpenBookContent { hand: minecraft_data::data::VarInt ,

 } impl PacketContent for PacketOpenBookContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.hand.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let hand : minecraft_data::data::VarInt = PacketContent :: read ( reader ) ?;;

 Ok ( Self { hand } ) } }
