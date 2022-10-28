use minecraft_protocol :: protocol :: PacketContent ; use minecraft_protocol :: protocol :: PacketSwitch ; use minecraft_protocol :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketFlying ; impl Packet for SbPacketFlying { type PacketIDType = i32 ; type PacketContent = PacketFlyingContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 20 } } pub struct PacketFlyingContent { pub on_ground: bool ,

 } impl PacketContent for PacketFlyingContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.on_ground.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let on_ground : bool = PacketContent :: read ( reader ) ?;;

 Ok ( Self { on_ground } ) } }
