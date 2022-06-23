mod sb_packet_set_difficulty { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketSetDifficulty ; impl Packet for SbPacketSetDifficulty { type PacketIDType = i32 ; type PacketContent = PacketSetDifficultyContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 2 } } pub struct PacketSetDifficultyContent { new_difficulty: u8 ,

 } impl PacketContent for PacketSetDifficultyContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.new_difficulty.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let new_difficulty : u8 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { new_difficulty } ) } }

 }

 pub use sb_packet_set_difficulty ::*;
