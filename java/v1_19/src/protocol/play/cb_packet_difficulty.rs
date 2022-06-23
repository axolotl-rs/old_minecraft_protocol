mod cb_packet_difficulty { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketDifficulty ; impl Packet for CbPacketDifficulty { type PacketIDType = i32 ; type PacketContent = PacketDifficultyContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 14 } } pub struct PacketDifficultyContent { difficulty: u8 ,

difficulty_locked: bool ,

 } impl PacketContent for PacketDifficultyContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.difficulty.write(writer)?;;

total_bytes += self.difficulty_locked.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let difficulty : u8 = PacketContent :: read ( reader ) ?;;

let difficulty_locked : bool = PacketContent :: read ( reader ) ?;;

 Ok ( Self { difficulty, difficulty_locked } ) } }

 }

 pub use cb_packet_difficulty ::*;
