mod sb_packet_lock_difficulty { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketLockDifficulty ; impl Packet for SbPacketLockDifficulty { type PacketIDType = i32 ; type PacketContent = PacketLockDifficultyContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 16 } } pub struct PacketLockDifficultyContent { locked: bool ,

 } impl PacketContent for PacketLockDifficultyContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.locked.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let locked : bool = PacketContent :: read ( reader ) ?;;

 Ok ( Self { locked } ) } }

 }

 pub use sb_packet_lock_difficulty ::*;
