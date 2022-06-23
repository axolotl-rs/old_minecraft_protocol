use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketWorldBorderSize ; impl Packet for CbPacketWorldBorderSize { type PacketIDType = i32 ; type PacketContent = PacketWorldBorderSizeContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 68 } } pub struct PacketWorldBorderSizeContent { diameter: f64 ,

 } impl PacketContent for PacketWorldBorderSizeContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.diameter.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let diameter : f64 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { diameter } ) } }
