use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketSetTitleSubtitle ; impl Packet for CbPacketSetTitleSubtitle { type PacketIDType = i32 ; type PacketContent = PacketSetTitleSubtitleContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 88 } } pub struct PacketSetTitleSubtitleContent { pub text: String ,

 } impl PacketContent for PacketSetTitleSubtitleContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.text.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let text : String = PacketContent :: read ( reader ) ?;;

 Ok ( Self { text } ) } }
