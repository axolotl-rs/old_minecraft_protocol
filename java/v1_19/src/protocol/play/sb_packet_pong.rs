use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketPong ; impl Packet for SbPacketPong { type PacketIDType = i32 ; type PacketContent = PacketPongContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 29 } } pub struct PacketPongContent { id: i32 ,

 } impl PacketContent for PacketPongContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.id.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let id : i32 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { id } ) } }
