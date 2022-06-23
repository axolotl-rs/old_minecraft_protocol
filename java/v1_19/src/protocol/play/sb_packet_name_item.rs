use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketNameItem ; impl Packet for SbPacketNameItem { type PacketIDType = i32 ; type PacketContent = PacketNameItemContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 32 } } pub struct PacketNameItemContent { name: String ,

 } impl PacketContent for PacketNameItemContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.name.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let name : String = PacketContent :: read ( reader ) ?;;

 Ok ( Self { name } ) } }
