mod sb_packet_abilities { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketAbilities ; impl Packet for SbPacketAbilities { type PacketIDType = i32 ; type PacketContent = PacketAbilitiesContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 25 } } pub struct PacketAbilitiesContent { flags: i8 ,

 } impl PacketContent for PacketAbilitiesContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.flags.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let flags : i8 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { flags } ) } }

 }

 pub use sb_packet_abilities ::*;
