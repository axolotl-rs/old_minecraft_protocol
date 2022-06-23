mod sb_packet_arm_animation { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketArmAnimation ; impl Packet for SbPacketArmAnimation { type PacketIDType = i32 ; type PacketContent = PacketArmAnimationContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 44 } } pub struct PacketArmAnimationContent { hand: minecraft_data::common::data::VarInt ,

 } impl PacketContent for PacketArmAnimationContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.hand.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let hand : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

 Ok ( Self { hand } ) } }

 }

 pub use sb_packet_arm_animation ::*;
