use minecraft_protocol :: protocol :: PacketContent ; use minecraft_protocol :: protocol :: PacketSwitch ; use minecraft_protocol :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketResourcePackReceive ; impl Packet for SbPacketResourcePackReceive { type PacketIDType = i32 ; type PacketContent = PacketResourcePackReceiveContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 33 } } pub struct PacketResourcePackReceiveContent { pub result: minecraft_protocol::data::VarInt ,

 } impl PacketContent for PacketResourcePackReceiveContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.result.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let result : minecraft_protocol::data::VarInt = PacketContent :: read ( reader ) ?;;

 Ok ( Self { result } ) } }
