use minecraft_protocol :: protocol :: PacketContent ; use minecraft_protocol :: protocol :: PacketSwitch ; use minecraft_protocol :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketOpenSignEntity ; impl Packet for CbPacketOpenSignEntity { type PacketIDType = i32 ; type PacketContent = PacketOpenSignEntityContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 47 } } pub struct PacketOpenSignEntityContent { pub location: minecraft_protocol::data::position::Position ,

 } impl PacketContent for PacketOpenSignEntityContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.location.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let location : minecraft_protocol::data::position::Position = PacketContent :: read ( reader ) ?;;

 Ok ( Self { location } ) } }
