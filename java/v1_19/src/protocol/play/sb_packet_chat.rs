use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketChat ; impl Packet for SbPacketChat { type PacketIDType = i32 ; type PacketContent = PacketChatContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 3 } } pub struct PacketChatContent { pub message: String ,

 } impl PacketContent for PacketChatContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.message.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let message : String = PacketContent :: read ( reader ) ?;;

 Ok ( Self { message } ) } }
