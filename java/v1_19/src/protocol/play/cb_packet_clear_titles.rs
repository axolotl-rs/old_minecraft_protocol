use minecraft_protocol :: protocol :: PacketContent ; use minecraft_protocol :: protocol :: PacketSwitch ; use minecraft_protocol :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketClearTitles ; impl Packet for CbPacketClearTitles { type PacketIDType = i32 ; type PacketContent = PacketClearTitlesContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 16 } } pub struct PacketClearTitlesContent { pub reset: bool ,

 } impl PacketContent for PacketClearTitlesContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.reset.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let reset : bool = PacketContent :: read ( reader ) ?;;

 Ok ( Self { reset } ) } }
