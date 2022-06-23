mod sb_packet_flying { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketFlying ; impl Packet for SbPacketFlying { type PacketIDType = i32 ; type PacketContent = PacketFlyingContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 20 } } pub struct PacketFlyingContent { on_ground: bool ,

 } impl PacketContent for PacketFlyingContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.on_ground.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let on_ground : bool = PacketContent :: read ( reader ) ?;;

 Ok ( Self { on_ground } ) } }

 }

 pub use sb_packet_flying ::*;
