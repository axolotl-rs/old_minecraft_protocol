use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketKeepAlive ; impl Packet for CbPacketKeepAlive { type PacketIDType = i32 ; type PacketContent = PacketKeepAliveContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 33 } } pub struct PacketKeepAliveContent { keep_alive_id: i64 ,

 } impl PacketContent for PacketKeepAliveContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.keep_alive_id.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let keep_alive_id : i64 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { keep_alive_id } ) } }
