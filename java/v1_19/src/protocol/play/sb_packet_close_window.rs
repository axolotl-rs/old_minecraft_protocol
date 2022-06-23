use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketCloseWindow ; impl Packet for SbPacketCloseWindow { type PacketIDType = i32 ; type PacketContent = PacketCloseWindowContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 9 } } pub struct PacketCloseWindowContent { window_id: u8 ,

 } impl PacketContent for PacketCloseWindowContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.window_id.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let window_id : u8 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { window_id } ) } }
