use minecraft_protocol :: protocol :: PacketContent ; use minecraft_protocol :: protocol :: PacketSwitch ; use minecraft_protocol :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketActionBar ; impl Packet for CbPacketActionBar { type PacketIDType = i32 ; type PacketContent = PacketActionBarContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 65 } } pub struct PacketActionBarContent { pub text: String ,

 } impl PacketContent for PacketActionBarContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.text.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let text : String = PacketContent :: read ( reader ) ?;;

 Ok ( Self { text } ) } }
